use ash::{version::DeviceV1_0, Device};

pub struct ManagedLogicalDevice {
    device_raw: Device,
}

impl ManagedLogicalDevice {
    pub fn new(device_raw: Device) -> Self {
        ManagedLogicalDevice { device_raw }
    }
}

impl Drop for ManagedLogicalDevice {
    fn drop(&mut self) {
        unsafe { self.device_raw.destroy_device(None) };
        trace!("Logical device was destroyed")
    }
}
