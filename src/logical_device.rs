//! 論理デバイス関連

use super::layer;
use super::queue_family;
use ash::{
    version::{DeviceV1_0, InstanceV1_0},
    vk::{DeviceCreateInfo, DeviceQueueCreateInfo, PhysicalDevice, PhysicalDeviceFeatures, Queue},
    Device, Instance,
};

/// 指定された物理デバイスをもとに論理デバイスを生成し、選択したキューファミリからグラフィックスキューを取得する
pub fn create_logical_device_with_graphics_queue(
    instance: &Instance,
    device: PhysicalDevice,
) -> (Device, Queue) {
    let queue_family_index = queue_family::find_queue_families(instance, &device).unwrap();
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
            .create_device(device, &device_create_info, None)
            .unwrap()
    };
    let graphics_queue = unsafe { device.get_device_queue(queue_family_index, 0) };
    (device, graphics_queue)
}
