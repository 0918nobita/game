//! 論理デバイス・グラフィックキュー関連

use super::layer;
use anyhow::Context;
use ash::{
    extensions::khr::Surface,
    version::{DeviceV1_0, InstanceV1_0},
    vk::{
        DeviceCreateInfo, DeviceQueueCreateInfo, PhysicalDevice, PhysicalDeviceFeatures, Queue,
        QueueFlags, SurfaceKHR,
    },
    Device, Instance,
};
use std::ffi::CStr;

pub fn create_logical_device_and_graphics_queue(
    instance: &Instance,
    surface: &Surface,
    surface_khr: SurfaceKHR,
) -> anyhow::Result<(Device, Queue)> {
    let mut physical_device_and_queue_family_index: Option<(PhysicalDevice, u32)> = None;

    for physical_device in unsafe { instance.enumerate_physical_devices().unwrap() }.into_iter() {
        let props =
            unsafe { instance.get_physical_device_queue_family_properties(physical_device) };
        for (index, family) in props.iter().enumerate() {
            if family.queue_count <= 0 || !family.queue_flags.contains(QueueFlags::GRAPHICS) {
                break;
            }
            let index = index as u32;
            let result = unsafe {
                surface.get_physical_device_surface_support(physical_device, index, surface_khr)
            };
            if let Ok(false) | Err(_) = result {
                break;
            }
            physical_device_and_queue_family_index = Some((physical_device, index));
        }
    }

    let (physical_device, queue_family_index) =
        physical_device_and_queue_family_index.expect("No suitable physical device");

    {
        let props = unsafe { instance.get_physical_device_properties(physical_device) };
        let device_name = unsafe { CStr::from_ptr(props.device_name.as_ptr()) };
        let device_name = device_name.to_str().unwrap().to_owned();
        debug!("Selected physical device: {}", device_name);
    }

    let queue_priorities = [1.0f32];
    let queue_create_infos = [DeviceQueueCreateInfo::builder()
        .queue_family_index(queue_family_index)
        .queue_priorities(&queue_priorities)
        .build()];
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

    let graphics_queue = unsafe { device.get_device_queue(queue_family_index, 0) };

    Ok((device, graphics_queue))
}
