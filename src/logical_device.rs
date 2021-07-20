use crate::{
    command_pool::CommandPool,
    queue_family_index::{Graphics, QueueFamilyIndex},
};
use anyhow::Context;
use ash::version::DeviceV1_0;
use std::rc::Rc;

pub struct LogicalDevice {
    raw: ash::Device,
}

impl LogicalDevice {
    pub fn new(raw: ash::Device) -> Rc<Self> {
        trace!("[CREATED] Logical device");
        Rc::new(LogicalDevice { raw })
    }

    pub fn create_command_pool(
        self: Rc<Self>,
        graphics_queue_family: &QueueFamilyIndex<Graphics>,
    ) -> anyhow::Result<CommandPool> {
        let create_info =
            ash::vk::CommandPoolCreateInfo::builder().queue_family_index(**graphics_queue_family);
        let command_pool_raw = unsafe { self.raw.create_command_pool(&create_info, None) }
            .context("Failed to create CommandPool")?;
        Ok(CommandPool::new(Box::new(move || unsafe {
            self.raw.destroy_command_pool(command_pool_raw, None)
        })))
    }
}

impl Drop for LogicalDevice {
    fn drop(&mut self) {
        unsafe { self.raw.destroy_device(None) };
        trace!("[DESTROYED] Logical device");
    }
}
