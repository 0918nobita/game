mod command_buffer;
mod command_pool;

use self::command_pool::ManagedCommandPool;
use anyhow::Context;
use ash::{
    version::{DeviceV1_0, InstanceV1_0},
    vk::{
        Extent3D, Format, ImageCreateInfo, ImageLayout, ImageTiling, ImageType, ImageUsageFlags,
        MemoryAllocateInfo, MemoryPropertyFlags, PhysicalDevice, Queue, SampleCountFlags,
        SharingMode,
    },
    Device, Instance,
};

pub struct ManagedLogicalDevice<'a> {
    instance_raw: &'a Instance,
    physical_device: PhysicalDevice,
    device_raw: Device,
    queue_indices: Vec<u32>,
}

impl<'a> ManagedLogicalDevice<'a> {
    pub fn new(
        instance_raw: &'a Instance,
        physical_device: PhysicalDevice,
        device_raw: Device,
        queue_indices: Vec<u32>,
    ) -> ManagedLogicalDevice<'a> {
        // 三角形を画像を描画するのが直近の目標なので、グラフィックスキューだけ利用して表示キューは放置
        ManagedLogicalDevice {
            instance_raw,
            physical_device,
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
        // 物理デバイスが持っているメモリについての情報
        let memory_properties = unsafe {
            self.instance_raw
                .get_physical_device_memory_properties(self.physical_device)
        };
        let image = unsafe { self.device_raw.create_image(&create_info, None) }
            .context("Failed to create image")?;
        // イメージに対してどんな種類のメモリがどれくらいのサイズ必要か
        let memory_requirements = unsafe { self.device_raw.get_image_memory_requirements(image) };
        let memory_type_index = memory_properties
            .memory_types
            .iter()
            .enumerate()
            .find_map(|(index, memory_type)| {
                let index = index as u32;
                (memory_requirements.memory_type_bits & 2u32.pow(index) != 0
                    && memory_type
                        .property_flags
                        .contains(MemoryPropertyFlags::DEVICE_LOCAL))
                .then(|| index)
            })
            .context("No suitable memory type")?;
        unsafe {
            let memory = self
                .device_raw
                .allocate_memory(
                    &MemoryAllocateInfo::builder()
                        .allocation_size(memory_requirements.size)
                        .memory_type_index(memory_type_index)
                        .build(),
                    None,
                )
                .context("Failed to allocate memory for image")?;
            self.device_raw
                .bind_image_memory(image, memory, 0)
                .context("Failed to bind device memory to image")?;
            self.device_raw.destroy_image(image, None);
            self.device_raw.free_memory(memory, None);
        }
        Ok(())
    }
}

impl Drop for ManagedLogicalDevice<'_> {
    fn drop(&mut self) {
        unsafe { self.device_raw.destroy_device(None) };
        trace!("Logical device was destroyed")
    }
}
