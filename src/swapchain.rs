use super::window;
use anyhow::Context;
use ash::{
    extensions::khr::{Surface, Swapchain},
    vk::{
        self, ColorSpaceKHR, CompositeAlphaFlagsKHR, Extent2D, ImageUsageFlags, PhysicalDevice,
        PresentModeKHR, SharingMode, SurfaceCapabilitiesKHR, SurfaceFormatKHR, SurfaceKHR,
        SwapchainCreateInfoKHR, SwapchainKHR,
    },
    Device, Instance,
};

pub struct SwapchainWrapper {
    swapchain_raw: Swapchain,
    swapchain_khr: SwapchainKHR,
    _images: Vec<vk::Image>,
}

impl SwapchainWrapper {
    pub fn new(
        instance: &Instance,
        surface: &Surface,
        surface_khr: SurfaceKHR,
        physical_device: PhysicalDevice,
        logical_device: &Device,
        image_sharing_mode: SharingMode,
        queue_family_indices: &[u32],
    ) -> anyhow::Result<Self> {
        let capabilities = unsafe {
            surface.get_physical_device_surface_capabilities(physical_device, surface_khr)
        }
        .context("Failed to get surface capabilities of physical device")?;
        let extent2d = decide_swapchain_extent(capabilities);
        debug!(
            "Window surface extent: {:?} x {:?}",
            extent2d.width, extent2d.height
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
            unsafe { surface.get_physical_device_surface_formats(physical_device, surface_khr) }
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
            surface.get_physical_device_surface_present_modes(physical_device, surface_khr)
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

        let create_info = SwapchainCreateInfoKHR::builder()
            .surface(surface_khr)
            .min_image_count(image_count)
            .image_format(format.format)
            .image_color_space(format.color_space)
            .image_extent(extent2d)
            .image_array_layers(1)
            .image_usage(ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(image_sharing_mode)
            .queue_family_indices(&queue_family_indices)
            .pre_transform(capabilities.current_transform)
            .composite_alpha(CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode)
            .clipped(true)
            .build();

        let swapchain_raw = Swapchain::new(instance, logical_device);
        let swapchain_khr = unsafe { swapchain_raw.create_swapchain(&create_info, None) }
            .context("Failed to create SwapchainKHR")?;
        trace!("SwapchainKHR was created");

        let images = unsafe { swapchain_raw.get_swapchain_images(swapchain_khr) }
            .context("Failed to get swapchain images")?;

        Ok(SwapchainWrapper {
            swapchain_raw,
            swapchain_khr,
            _images: images,
        })
    }
}

impl Drop for SwapchainWrapper {
    fn drop(&mut self) {
        unsafe {
            self.swapchain_raw
                .destroy_swapchain(self.swapchain_khr, None)
        }
        trace!("SwapchainKHR was destroyed")
    }
}

fn decide_swapchain_extent(capabilities: SurfaceCapabilitiesKHR) -> Extent2D {
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
