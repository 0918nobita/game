use crate::queue_family_index::{Graphics, QueueFamilyIndex};

pub struct PhysicalDevice {
    pub device_type: ash::vk::PhysicalDeviceType,
    pub device_name: String,
    pub graphics_queue_family: QueueFamilyIndex<Graphics>,
}

impl std::fmt::Debug for PhysicalDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} (DeviceType: {:?}, GraphicsQueueFamily: {:?})",
            self.device_name, self.device_type, self.graphics_queue_family
        )
    }
}
