use anyhow::Context;
use ash::{
    version::{DeviceV1_0, InstanceV1_0},
    vk::{
        ComponentMapping, ComponentSwizzle, DeviceMemory, Extent3D, Format, Image,
        ImageAspectFlags, ImageCreateInfo, ImageLayout, ImageSubresourceRange, ImageTiling,
        ImageType, ImageUsageFlags, ImageView, ImageViewCreateInfo, ImageViewType,
        MemoryAllocateInfo, MemoryPropertyFlags, PhysicalDevice, SampleCountFlags, SharingMode,
    },
    Device, Instance,
};

pub struct ManagedImage<'a> {
    device_raw: &'a Device,
    device_memory_raw: DeviceMemory,
    image_raw: Image,
    image_view: ImageView,
}

impl<'a> ManagedImage<'a> {
    pub fn new(
        instance_raw: &Instance,
        physical_device: &PhysicalDevice,
        device_raw: &'a Device,
        width: u32,
        height: u32,
    ) -> anyhow::Result<ManagedImage<'a>> {
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
        let memory_properties =
            unsafe { instance_raw.get_physical_device_memory_properties(*physical_device) };
        let image_raw = unsafe { device_raw.create_image(&create_info, None) }
            .context("Failed to create image")?;
        // イメージに対してどんな種類のメモリがどれくらいのサイズ必要か
        let memory_requirements = unsafe { device_raw.get_image_memory_requirements(image_raw) };
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
        let device_memory_raw = unsafe {
            device_raw.allocate_memory(
                &MemoryAllocateInfo::builder()
                    .allocation_size(memory_requirements.size)
                    .memory_type_index(memory_type_index)
                    .build(),
                None,
            )
        }
        .context("Failed to allocate memory for image")?;
        unsafe { device_raw.bind_image_memory(image_raw, device_memory_raw, 0) }
            .context("Failed to bind device memory to image")?;
        let image_view_create_info = ImageViewCreateInfo::builder()
            .image(image_raw)
            .view_type(ImageViewType::TYPE_2D)
            .format(Format::R8G8B8A8_UNORM)
            .components(
                ComponentMapping::builder()
                    .r(ComponentSwizzle::IDENTITY)
                    .g(ComponentSwizzle::IDENTITY)
                    .b(ComponentSwizzle::IDENTITY)
                    .a(ComponentSwizzle::IDENTITY)
                    .build(),
            )
            .subresource_range(
                ImageSubresourceRange::builder()
                    .aspect_mask(ImageAspectFlags::COLOR)
                    .base_mip_level(0)
                    .level_count(1)
                    .base_array_layer(0)
                    .layer_count(1)
                    .build(),
            )
            .build();
        let image_view = unsafe { device_raw.create_image_view(&image_view_create_info, None) }
            .context("Failed to create ImageView")?;
        Ok(ManagedImage {
            device_raw,
            device_memory_raw,
            image_raw,
            image_view,
        })
    }
}

impl Drop for ManagedImage<'_> {
    fn drop(&mut self) {
        unsafe {
            self.device_raw.destroy_image_view(self.image_view, None);
            trace!("ImageView was destroyed");
            self.device_raw.destroy_image(self.image_raw, None);
            trace!("Image was destroyed");
            self.device_raw.free_memory(self.device_memory_raw, None);
            trace!("GPU memory allocated for image was released");
        }
    }
}
