mod command_buffer;
mod command_pool;

use self::command_pool::ManagedCommandPool;
use anyhow::Context;
use ash::{
    version::DeviceV1_0,
    vk::{
        Extent3D, Format, ImageCreateInfo, ImageLayout, ImageTiling, ImageType, ImageUsageFlags,
        Queue, SampleCountFlags, SharingMode,
    },
    Device,
};

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

    pub fn get_graphics_queue(&self) -> Queue {
        let graphics_queue_family_index = self.queue_indices[0];
        unsafe {
            self.device_raw
                .get_device_queue(graphics_queue_family_index, 0)
        }
    }

    pub fn create_command_pool(&self) -> anyhow::Result<ManagedCommandPool> {
        let graphics_queue_family_index = self.queue_indices[0];
        ManagedCommandPool::new(&self.device_raw, graphics_queue_family_index)
    }

    pub fn create_image(&self, width: u32, height: u32) -> anyhow::Result<()> {
        let create_info = ImageCreateInfo::builder()
            .image_type(ImageType::TYPE_2D)
            .extent(
                Extent3D::builder()
                    .width(width)
                    .height(height)
                    .depth(1)
                    .build(),
            )
            .mip_levels(1)
            .array_layers(1)
            .format(Format::R8G8B8A8_UNORM)
            .tiling(ImageTiling::OPTIMAL)
            .initial_layout(ImageLayout::UNDEFINED)
            .usage(ImageUsageFlags::COLOR_ATTACHMENT)
            .sharing_mode(SharingMode::EXCLUSIVE)
            .samples(SampleCountFlags::TYPE_1)
            .build();
        let image = unsafe { self.device_raw.create_image(&create_info, None) }
            .context("Failed to create image")?;
        unsafe { self.device_raw.destroy_image(image, None) };
        Ok(())
    }
}

impl Drop for ManagedLogicalDevice {
    fn drop(&mut self) {
        unsafe { self.device_raw.destroy_device(None) };
        trace!("Logical device was destroyed")
    }
}
