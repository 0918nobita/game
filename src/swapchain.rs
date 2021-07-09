use crate::window;
use anyhow::Context;
use ash::{
    extensions::khr::{Surface, Swapchain},
    version::DeviceV1_0,
    vk::{self, ColorSpaceKHR, Extent2D, PhysicalDevice, PresentModeKHR, SurfaceFormatKHR},
    Device, Instance,
};

pub struct SwapchainWrapper<'a> {
    logical_device: &'a Device,
    swapchain_raw: Swapchain,
    swapchain_khr: vk::SwapchainKHR,
    image_views: Vec<vk::ImageView>,
}

impl<'a> SwapchainWrapper<'a> {
    pub fn new(
        instance: &Instance,
        surface: &Surface,
        surface_khr: vk::SurfaceKHR,
        physical_device: &PhysicalDevice,
        logical_device: &'a Device,
        image_sharing_mode: vk::SharingMode,
        queue_family_indices: &[u32],
    ) -> anyhow::Result<SwapchainWrapper<'a>> {
        let capabilities = unsafe {
            surface.get_physical_device_surface_capabilities(*physical_device, surface_khr)
        }
        .context("Failed to get surface capabilities of physical device")?;
        let image_extent = decide_swapchain_extent(capabilities);
        debug!(
            "Window surface extent: {:?} x {:?}",
            image_extent.width, image_extent.height
        );

        let image_count = {
            let max = capabilities.max_image_count;
            let preferred = capabilities.min_image_count + 1;
            if max > 0 && preferred > max {
                max
            } else {
                preferred
            }
        };

        let available_formats =
            unsafe { surface.get_physical_device_surface_formats(*physical_device, surface_khr) }
                .context("Failed to get surface formats of physical device")?;
        trace!("Available pixel formats:");
        for f in available_formats.iter() {
            trace!(
                "    - PixelFormat: {:?}, ColorSpace: {:?}",
                f.format,
                f.color_space
            )
        }

        let format = choose_swapchain_surface_format(&available_formats);
        debug!("Pixel format: {:?}", format.format);
        debug!("Color space: {:?}", format.color_space);

        let available_present_modes = unsafe {
            surface.get_physical_device_surface_present_modes(*physical_device, surface_khr)
        }
        .context("Failed to get present modes of physical device")?;
        trace!(
            "Available present modes: {}",
            available_present_modes
                .iter()
                .map(|m| format!("{:?}", m))
                .collect::<Vec<_>>()
                .join(", ")
        );

        let present_mode = choose_swapchain_surface_present_mode(&available_present_modes);
        debug!("Present mode: {:?}", present_mode);

        let create_info = vk::SwapchainCreateInfoKHR::builder()
            .surface(surface_khr)
            .min_image_count(image_count)
            .image_format(format.format)
            .image_color_space(format.color_space)
            .image_extent(image_extent)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(image_sharing_mode)
            .queue_family_indices(&queue_family_indices)
            .pre_transform(capabilities.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode)
            .clipped(true)
            .build();

        let swapchain_raw = Swapchain::new(instance, logical_device);
        let swapchain_khr = unsafe { swapchain_raw.create_swapchain(&create_info, None) }
            .context("Failed to create SwapchainKHR")?;
        trace!("SwapchainKHR was created");

        let images = unsafe { swapchain_raw.get_swapchain_images(swapchain_khr) }
            .context("Failed to get swapchain images")?;

        // パイプラインからイメージを利用するために必要
        let image_views = images
            .into_iter()
            .map(|image| {
                let image_view_create_info = vk::ImageViewCreateInfo::builder()
                    .image(image)
                    .view_type(vk::ImageViewType::TYPE_2D)
                    .format(format.format)
                    // 画像の用途や、画像のどの部分にアクセスするかを指定する
                    // ミップマッピングレベルや複数レイヤを使用せず、カラーターゲットとして使用する
                    .subresource_range(vk::ImageSubresourceRange {
                        aspect_mask: vk::ImageAspectFlags::COLOR,
                        base_mip_level: 0,
                        level_count: 1,
                        base_array_layer: 0,
                        layer_count: 1,
                    })
                    .build();
                unsafe { logical_device.create_image_view(&image_view_create_info, None) }.unwrap()
            })
            .collect::<Vec<_>>();
        debug!("The number of image views: {}", image_views.len());

        Ok(SwapchainWrapper {
            logical_device,
            swapchain_raw,
            swapchain_khr,
            image_views,
        })
    }

    pub fn get_image_count(&self) -> anyhow::Result<usize> {
        unsafe { self.swapchain_raw.get_swapchain_images(self.swapchain_khr) }
            .context("Failed to get swapchain images ")
            .map(|imgs| imgs.len())
    }
}

impl<'a> Drop for SwapchainWrapper<'a> {
    fn drop(&mut self) {
        for image_view in self.image_views.iter() {
            unsafe { self.logical_device.destroy_image_view(*image_view, None) }
        }
        trace!("All image views included in swapchain were destroyed");
        unsafe {
            self.swapchain_raw
                .destroy_swapchain(self.swapchain_khr, None)
        }
        trace!("SwapchainKHR was destroyed")
    }
}

fn decide_swapchain_extent(capabilities: vk::SurfaceCapabilitiesKHR) -> Extent2D {
    if capabilities.current_extent.width != std::u32::MAX {
        return capabilities.current_extent;
    }
    let min = capabilities.min_image_extent;
    let max = capabilities.max_image_extent;
    let width = window::WIDTH.max(max.width).min(min.width);
    let height = window::HEIGHT.max(max.height).min(min.height);
    Extent2D { width, height }
}

fn choose_swapchain_surface_format(available_formats: &[SurfaceFormatKHR]) -> SurfaceFormatKHR {
    if available_formats.len() == 1 && available_formats[0].format == vk::Format::UNDEFINED {
        return SurfaceFormatKHR {
            format: vk::Format::B8G8R8A8_UNORM,
            color_space: ColorSpaceKHR::SRGB_NONLINEAR,
        };
    }
    *available_formats
        .iter()
        .find(|format| {
            format.format == vk::Format::B8G8R8A8_UNORM
                && format.color_space == ColorSpaceKHR::SRGB_NONLINEAR
        })
        .unwrap_or(&available_formats[0])
}

fn choose_swapchain_surface_present_mode(
    available_present_modes: &[PresentModeKHR],
) -> PresentModeKHR {
    if available_present_modes.contains(&PresentModeKHR::MAILBOX) {
        PresentModeKHR::MAILBOX
    } else if available_present_modes.contains(&PresentModeKHR::FIFO) {
        PresentModeKHR::FIFO
    } else {
        PresentModeKHR::IMMEDIATE
    }
}
