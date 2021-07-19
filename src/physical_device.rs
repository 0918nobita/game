pub struct PhysicalDevice {
    device_type: ash::vk::PhysicalDeviceType,
    device_name: String,
}

impl PhysicalDevice {
    pub fn new(device_type: ash::vk::PhysicalDeviceType, device_name: String) -> Self {
        PhysicalDevice {
            device_type,
            device_name,
        }
    }
}

impl std::fmt::Debug for PhysicalDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({:?})", self.device_name, self.device_type)
    }
}
