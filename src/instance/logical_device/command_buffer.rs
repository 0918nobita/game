use ash::{
    version::DeviceV1_0,
    vk::{CommandBuffer, CommandBufferBeginInfo, CommandPool, Fence, Queue, SubmitInfo},
    Device,
};

pub struct ManagedCommandBuffer<'a> {
    device_raw: &'a Device,
    command_pool_raw: &'a CommandPool,
    command_buffer_raw: CommandBuffer,
}

impl<'a> ManagedCommandBuffer<'a> {
    pub fn new(
        device_raw: &'a Device,
        command_pool_raw: &'a CommandPool,
        command_buffer_raw: CommandBuffer,
    ) -> ManagedCommandBuffer<'a> {
        ManagedCommandBuffer {
            device_raw,
            command_pool_raw,
            command_buffer_raw,
        }
    }

    pub fn submit_empty_cmd(&self, queue: &Queue) -> anyhow::Result<()> {
        let begin_info = CommandBufferBeginInfo::builder().build();
        let submit_info = SubmitInfo::builder()
            .command_buffers(&[self.command_buffer_raw])
            .build();
        unsafe {
            self.device_raw
                .begin_command_buffer(self.command_buffer_raw, &begin_info)?;
            self.device_raw
                .end_command_buffer(self.command_buffer_raw)?;
            self.device_raw
                .queue_submit(*queue, &[submit_info], Fence::null())?;
            self.device_raw.queue_wait_idle(*queue)?;
        }
        Ok(())
    }
}

impl Drop for ManagedCommandBuffer<'_> {
    fn drop(&mut self) {
        unsafe {
            self.device_raw
                .free_command_buffers(*self.command_pool_raw, &[self.command_buffer_raw])
        }
    }
}
