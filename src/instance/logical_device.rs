mod command_pool;

use self::command_pool::ManagedCommandPool;
use ash::{version::DeviceV1_0, Device};

pub struct ManagedLogicalDevice {
    device_raw: Device,
    queue_indices: Vec<u32>,
}

impl ManagedLogicalDevice {
    pub fn new(device_raw: Device, queue_indices: Vec<u32>) -> Self {
        // 三角形を画像を描画するのが直近の目標なので、グラフィックスキューだけ利用して表示キューは放置
        ManagedLogicalDevice {
            device_raw,
            queue_indices,
        }
    }

    pub fn create_command_pool(&self) -> anyhow::Result<ManagedCommandPool> {
        let graphics_queue_family_index = self.queue_indices[0];
        let _graphics_queue = unsafe {
            self.device_raw
                .get_device_queue(graphics_queue_family_index, 0)
        };
        ManagedCommandPool::new(&self.device_raw, graphics_queue_family_index)
    }
}

impl Drop for ManagedLogicalDevice {
    fn drop(&mut self) {
        unsafe { self.device_raw.destroy_device(None) };
        trace!("Logical device was destroyed")
    }
}
