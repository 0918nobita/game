//! 開発中のインディーゲーム

#[macro_use]
extern crate log;
mod instance;
mod layer;
mod logical_device;
mod physical_device;
mod queue_family;

use ash::{
    version::{DeviceV1_0, InstanceV1_0},
    vk::{PhysicalDevice, Queue},
    Device, Entry, Instance,
};

/// Vulkan インスタンスと、それがメモリ上に存在しているときに同時に存在している必要のあるデータをまとめたもの
pub struct Application {
    instance: Instance,
    logical_device: Device,
    /// `ash::Instance` を利用するためには、同時に `ash::Entry` が Drop されずに存在している必要がある
    _entry: Entry,
    _physical_device: PhysicalDevice,
    _graphics_queue: Queue,
}

impl Application {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        trace!("Initialization started");
        let entry = unsafe { Entry::new()? };
        let instance = instance::create_instance(&entry)?;
        let physical_device = physical_device::pick_physical_device(&instance);
        let (device, graphics_queue) =
            logical_device::create_logical_device_with_graphics_queue(&instance, physical_device);
        trace!("Initialization completed");
        Ok(Application {
            instance,
            logical_device: device,
            _entry: entry,
            _physical_device: physical_device,
            _graphics_queue: graphics_queue,
        })
    }
}

impl Drop for Application {
    fn drop(&mut self) {
        unsafe {
            self.logical_device.destroy_device(None);
            self.instance.destroy_instance(None);
        }
        trace!("Logical device and instance were released");
    }
}
