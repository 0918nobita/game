//! 論理デバイス・グラフィックキュー関連

use crate::layer;
use anyhow::Context;
use ash::{
    extensions::khr::{Surface, Swapchain},
    version::InstanceV1_0,
    vk::{
        DeviceCreateInfo, DeviceQueueCreateInfo, PhysicalDevice, PhysicalDeviceFeatures,
        QueueFlags, SurfaceKHR,
    },
    Device, Instance,
};
use std::ffi::CStr;

pub struct Queues {
    pub graphics: u32,
    pub presentation: u32,
}

/// 論理デバイス・グラフィックキュー・表示用キューを生成する
pub fn create_device_and_queue_indices(
    instance: &Instance,
    surface: &Surface,
    surface_khr: SurfaceKHR,
) -> anyhow::Result<(PhysicalDevice, Device, Queues)> {
    let physical_devices = unsafe { instance.enumerate_physical_devices() }
        .context("Failed to enumerate physical devices")?;
    trace!("Start searching suitable physical device");
    let (physical_device, queues) = physical_devices
        .into_iter()
        .find_map(|device| {
            let extension_props =
                unsafe { instance.enumerate_device_extension_properties(device) }.ok()?;
            extension_props.iter().find(|ext| {
                let name = unsafe { CStr::from_ptr(ext.extension_name.as_ptr()) };
                name == Swapchain::name()
            })?;
            find_suitable_queues(instance, surface, surface_khr, device)
                .map(|queues| (device, queues))
        })
        .expect("No suitable physical device");

    {
        let props = unsafe { instance.get_physical_device_properties(physical_device) };
        let device_name = unsafe { CStr::from_ptr(props.device_name.as_ptr()) };
        let device_name = device_name.to_str().unwrap().to_owned();
        debug!("Selected physical device: {}", device_name);
        debug!("Queue family index (Graphics): {}", queues.graphics);
        debug!("Queue family index (Presentation): {}", queues.presentation);
    }

    let queue_priorities = [1.0f32];
    let queue_create_infos: Vec<DeviceQueueCreateInfo> = {
        let mut indices = vec![queues.graphics, queues.presentation];
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
        .enabled_extension_names(&[Swapchain::name().as_ptr()])
        .enabled_features(&device_features)
        .enabled_layer_names(&layer_name_ptrs)
        .build();

    let device = unsafe {
        instance
            .create_device(physical_device, &device_create_info, None)
            .context("Failed to create logical device")?
    };

    Ok((physical_device, device, queues))
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
