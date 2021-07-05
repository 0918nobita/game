//! キューファミリ関連

use ash::{
    version::InstanceV1_0,
    vk::{PhysicalDevice, QueueFamilyProperties, QueueFlags},
    Instance,
};

/// 適切なキューファミリを選択し、そのインデックスを取得する
pub fn find_queue_families(instance: &Instance, device: &PhysicalDevice) -> Option<u32> {
    let props = unsafe { instance.get_physical_device_queue_family_properties(*device) };
    props
        .iter()
        .enumerate()
        .find(|(_, family)| is_queue_family_suitable(family))
        .map(|(index, _)| index as _)
}

/// 指定されたキューファミリが、1本以上のキューを持っていて、かつグラフィックパイプラインを使ったレンダリングに利用できるかどうかを調べる
pub fn is_queue_family_suitable(family: &QueueFamilyProperties) -> bool {
    family.queue_count > 0 && family.queue_flags.contains(QueueFlags::GRAPHICS)
}
