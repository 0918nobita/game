use super::command_buffer::ManagedCommandBuffer;
use ash::{
    version::DeviceV1_0,
    vk::{CommandBufferAllocateInfo, CommandBufferLevel, CommandPool, CommandPoolCreateInfo},
    Device,
};

pub struct ManagedCommandPool<'a> {
    device: &'a Device,
    command_pool_raw: CommandPool,
}

impl<'a> ManagedCommandPool<'a> {
    pub fn new(
        device: &'a Device,
        graphics_queue_family_index: u32,
    ) -> anyhow::Result<ManagedCommandPool<'a>> {
        let create_info = CommandPoolCreateInfo::builder()
            .queue_family_index(graphics_queue_family_index)
            .build();
        let command_pool_raw = unsafe { device.create_command_pool(&create_info, None) }?;
        Ok(ManagedCommandPool {
            device,
            command_pool_raw,
        })
    }

    pub fn allocate_command_buffer(&self) -> anyhow::Result<ManagedCommandBuffer> {
        let create_info = CommandBufferAllocateInfo::builder()
            .command_pool(self.command_pool_raw)
            .command_buffer_count(1)
            .level(CommandBufferLevel::PRIMARY)
            .build();
        let command_buffer = unsafe { self.device.allocate_command_buffers(&create_info) }?[0];
        Ok(ManagedCommandBuffer::new(
            self.device,
            &self.command_pool_raw,
            command_buffer,
        ))
    }
}

impl Drop for ManagedCommandPool<'_> {
    fn drop(&mut self) {
        unsafe {
            self.device
                .destroy_command_pool(self.command_pool_raw, None)
        };
        trace!("CommandPool was destroyed");
    }
}
