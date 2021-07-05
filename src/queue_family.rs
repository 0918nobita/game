use ash::{
    version::InstanceV1_0,
    vk::{PhysicalDevice, QueueFamilyProperties, QueueFlags},
    Instance,
};

pub fn find_queue_families(instance: &Instance, device: &PhysicalDevice) -> Option<u32> {
    let props = unsafe { instance.get_physical_device_queue_family_properties(*device) };
    props
        .iter()
        .enumerate()
        .find(|(_, family)| is_queue_family_suitable(family))
        .map(|(index, _)| index as _)
}

pub fn is_queue_family_suitable(family: &QueueFamilyProperties) -> bool {
    family.queue_count > 0 && family.queue_flags.contains(QueueFlags::GRAPHICS)
}
