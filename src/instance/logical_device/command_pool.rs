use ash::{
    version::DeviceV1_0,
    vk::{CommandPool, CommandPoolCreateInfo},
    Device,
};

pub struct ManagedCommandPool<'a> {
    device_raw: &'a Device,
    command_pool_raw: CommandPool,
}

impl<'a> ManagedCommandPool<'a> {
    pub fn new(
        device_raw: &'a Device,
        graphics_queue_family_index: u32,
    ) -> anyhow::Result<ManagedCommandPool<'a>> {
        let create_info = CommandPoolCreateInfo::builder()
            .queue_family_index(graphics_queue_family_index)
            .build();
        let command_pool_raw = unsafe { device_raw.create_command_pool(&create_info, None) }?;
        Ok(ManagedCommandPool {
            device_raw,
            command_pool_raw,
        })
    }
}

impl<'a> Drop for ManagedCommandPool<'a> {
    fn drop(&mut self) {
        unsafe {
            self.device_raw
                .destroy_command_pool(self.command_pool_raw, None)
        };
        trace!("CommandPool was destroyed");
    }
}
