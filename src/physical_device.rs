use anyhow::Context;
use ash::{version::InstanceV1_0, vk::PhysicalDevice, Instance};

pub struct PhysicalDeviceWrapper {
    physical_device_raw: PhysicalDevice,
}

impl PhysicalDeviceWrapper {
    pub fn find<P>(instance: &Instance, predicate: P) -> anyhow::Result<Self>
    where
        P: FnMut(&PhysicalDevice) -> bool,
    {
        unsafe { instance.enumerate_physical_devices() }
            .context("Failed to enumerate physical devices")?
            .into_iter()
            .find(predicate)
            .context("Failed to find suitable physical device")
            .map(|physical_device_raw| PhysicalDeviceWrapper {
                physical_device_raw,
            })
    }
}
