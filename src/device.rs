//! 論理デバイス・グラフィックキュー関連

use super::{layer, window};
use anyhow::Context;
use ash::{
    extensions::khr::{Surface, Swapchain},
    version::{DeviceV1_0, InstanceV1_0},
    vk::{
        ColorSpaceKHR, DeviceCreateInfo, DeviceQueueCreateInfo, Extent2D, Format, PhysicalDevice,
        PhysicalDeviceFeatures, Queue, QueueFlags, SurfaceCapabilitiesKHR, SurfaceFormatKHR,
        SurfaceKHR,
    },
    Device, Instance,
};
use std::ffi::CStr;

struct Queues {
    graphics: u32,
    presentation: u32,
}

/// 論理デバイス・グラフィックキュー・表示用キューを生成する
pub fn create_logical_device_and_queues(
    instance: &Instance,
    surface: &Surface,
    surface_khr: SurfaceKHR,
) -> anyhow::Result<(Device, Queue, Queue)> {
    let physical_devices = unsafe { instance.enumerate_physical_devices() }
        .context("Failed to enumerate physical devices")?;
    trace!("Start searching suitable physical device");
    let (
        physical_device,
        Queues {
            graphics,
            presentation,
        },
    ) = physical_devices
        .into_iter()
        .find_map(|device| {
            let extension_props =
                unsafe { instance.enumerate_device_extension_properties(device) }.ok()?;
            extension_props
                .iter()
                .any(|ext| {
                    let name = unsafe { CStr::from_ptr(ext.extension_name.as_ptr()) };
                    name == Swapchain::name()
                })
                .then(|| ())?;
            find_suitable_queues(instance, surface, surface_khr, device)
                .map(|queues| (device, queues))
        })
        .expect("No suitable physical device");

    {
        let props = unsafe { instance.get_physical_device_properties(physical_device) };
        let device_name = unsafe { CStr::from_ptr(props.device_name.as_ptr()) };
        let device_name = device_name.to_str().unwrap().to_owned();
        debug!("Selected physical device: {}", device_name);
        debug!("Queue family index (Graphics): {}", graphics);
        debug!("Queue family index (Presentation): {}", presentation);
    }

    let capabilities =
        unsafe { surface.get_physical_device_surface_capabilities(physical_device, surface_khr) }
            .context("Failed to get surface capabilities of physical device")?;
    let extent2d = decide_swapchain_extent(capabilities);
    debug!(
        "Window surface extent: {:?} x {:?}",
        extent2d.width, extent2d.height
    );

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

    let present_modes =
        unsafe { surface.get_physical_device_surface_present_modes(physical_device, surface_khr) }
            .context("Failed to get present modes of physical device")?;
    trace!(
        "Available present modes: {}",
        present_modes
            .iter()
            .map(|m| format!("{:?}", m))
            .collect::<Vec<_>>()
            .join(", ")
    );

    let queue_priorities = [1.0f32];
    let queue_create_infos: Vec<DeviceQueueCreateInfo> = {
        let mut indices = vec![graphics, presentation];
        indices.dedup();
        indices
            .iter()
            .map(|index| {
                DeviceQueueCreateInfo::builder()
                    .queue_family_index(*index)
                    .queue_priorities(&queue_priorities)
                    .build()
            })
            .collect()
    };
    let device_features = PhysicalDeviceFeatures::builder().build();
    let layer_name_ptrs = layer::get_layer_name_ptrs();
    let device_create_info = DeviceCreateInfo::builder()
        .queue_create_infos(&queue_create_infos)
        .enabled_features(&device_features)
        .enabled_layer_names(&layer_name_ptrs)
        .build();

    let device = unsafe {
        instance
            .create_device(physical_device, &device_create_info, None)
            .context("Failed to create logical device")?
    };

    let graphics_queue = unsafe { device.get_device_queue(graphics, 0) };
    let present_queue = unsafe { device.get_device_queue(presentation, 0) };

    Ok((device, graphics_queue, present_queue))
}

fn choose_swapchain_surface_format(available_formats: &[SurfaceFormatKHR]) -> SurfaceFormatKHR {
    if available_formats.len() == 1 && available_formats[0].format == Format::UNDEFINED {
        return SurfaceFormatKHR {
            format: Format::B8G8R8A8_UNORM,
            color_space: ColorSpaceKHR::SRGB_NONLINEAR,
        };
    }
    *available_formats
        .iter()
        .find(|format| {
            format.format == Format::B8G8R8A8_UNORM
                && format.color_space == ColorSpaceKHR::SRGB_NONLINEAR
        })
        .unwrap_or(&available_formats[0])
}

/// グラフィックキューと表示用キューをそれぞれ選択する
fn find_suitable_queues(
    instance: &Instance,
    surface: &Surface,
    surface_khr: SurfaceKHR,
    device: PhysicalDevice,
) -> Option<Queues> {
    let queue_family_props =
        unsafe { instance.get_physical_device_queue_family_properties(device) };

    let graphics = queue_family_props
        .iter()
        .enumerate()
        .find_map(|(index, family)| {
            (family.queue_count > 0 && family.queue_flags.contains(QueueFlags::GRAPHICS))
                .then(|| index as u32)
        })?;
    let presentation = queue_family_props
        .iter()
        .enumerate()
        .find_map(|(index, _)| {
            let index = index as u32;
            unsafe { surface.get_physical_device_surface_support(device, index, surface_khr) }
                .unwrap_or(false)
                .then(|| index)
        })?;
    Some(Queues {
        graphics,
        presentation,
    })
}

/// サーフェス制約をもとに、表示サイズを決定する
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
