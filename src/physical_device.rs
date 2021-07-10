use anyhow::Context;
use ash::{version::InstanceV1_0, vk::PhysicalDevice, Instance};

pub struct ManagedPhysicalDevice<'a> {
    instance: &'a Instance,
    physical_device_raw: PhysicalDevice,
}

impl<'a> ManagedPhysicalDevice<'a> {
    pub fn find<P>(instance: &'a Instance, predicate: P) -> anyhow::Result<Self>
    where
        P: FnMut(&PhysicalDevice) -> bool,
    {
        unsafe { instance.enumerate_physical_devices() }
            .context("Failed to enumerate physical devices")?
            .into_iter()
            .find(predicate)
            .context("Failed to find suitable physical device")
            .map(|physical_device_raw| ManagedPhysicalDevice {
                instance,
                physical_device_raw,
            })
    }
}
