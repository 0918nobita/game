//! 論理デバイス・グラフィックキュー関連

use super::layer;
use anyhow::Context;
use ash::{
    extensions::khr::{Surface, Swapchain},
    version::{DeviceV1_0, InstanceV1_0},
    vk::{
        DeviceCreateInfo, DeviceQueueCreateInfo, PhysicalDevice, PhysicalDeviceFeatures, Queue,
        QueueFlags, SurfaceKHR,
    },
    Device, Instance,
};
use std::ffi::CStr;

struct Queues {
    graphics: u32,
    presentation: u32,
}

pub fn create_logical_device_and_queues(
    instance: &Instance,
    surface: &Surface,
    surface_khr: SurfaceKHR,
) -> anyhow::Result<(Device, Queue, Queue)> {
    let physical_devices = unsafe { instance.enumerate_physical_devices() }
        .context("Failed to enumerate physical devices")?;
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
            let is_swapchain_supported = extension_props.iter().any(|ext| {
                let name = unsafe { CStr::from_ptr(ext.extension_name.as_ptr()) };
                name == Swapchain::name()
            });
            debug!(
                "Swapchain support: {}",
                if is_swapchain_supported { "yes" } else { "no" }
            );

            let capabilities =
                unsafe { surface.get_physical_device_surface_capabilities(device, surface_khr) }
                    .ok()?;
            debug!("Capabilities: {:?}", capabilities);

            let formats =
                unsafe { surface.get_physical_device_surface_formats(device, surface_khr) }.ok()?;
            debug!("Available pixel formats: {:?}", formats);

            let present_modes =
                unsafe { surface.get_physical_device_surface_present_modes(device, surface_khr) }
                    .ok()?;
            debug!("Available present modes: {:?}", present_modes);
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
