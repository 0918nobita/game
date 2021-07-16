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

pub struct ManagedAndLinearImage<'a> {
    device: &'a Device,
    device_memory: DeviceMemory,
    image_raw: Image,
    image_view: ImageView,
}

impl<'a> ManagedAndLinearImage<'a> {
    pub fn new(
        instance: &'a Instance,
        physical_device: &PhysicalDevice,
        device: &'a Device,
        width: u32,
        height: u32,
    ) -> anyhow::Result<ManagedAndLinearImage<'a>> {
        let create_info = ImageCreateInfo::builder()
            .image_type(ImageType::TYPE_2D)
            .extent(Extent3D {
                width,
                height,
                depth: 1,
            })
            .mip_levels(1)
            .array_layers(1)
            .format(Format::R8G8B8A8_UNORM)
            .tiling(ImageTiling::LINEAR)
            .initial_layout(ImageLayout::UNDEFINED)
            .usage(ImageUsageFlags::SAMPLED | ImageUsageFlags::TRANSFER_DST)
            .sharing_mode(SharingMode::EXCLUSIVE)
            .samples(SampleCountFlags::TYPE_1)
            .build();
        let image_raw = unsafe { device.create_image(&create_info, None) }
            .context("Failed to create linear image")?;
        let memory_properties =
            unsafe { instance.get_physical_device_memory_properties(*physical_device) };
        let memory_requirements = unsafe { device.get_image_memory_requirements(image_raw) };
        let memory_type_index = memory_properties
            .memory_types
            .iter()
            .enumerate()
            .find_map(|(index, memory_type)| {
                let index = index as u32;
                (memory_requirements.memory_type_bits & 2u32.pow(index) != 0
                    && memory_type
                        .property_flags
                        .contains(MemoryPropertyFlags::HOST_VISIBLE))
                .then(|| index)
            })
            .context("No suitable memory type")?;
        let device_memory = unsafe {
            device.allocate_memory(
                &MemoryAllocateInfo::builder()
                    .allocation_size(memory_requirements.size)
                    .memory_type_index(memory_type_index)
                    .build(),
                None,
            )
        }
        .context("Failed to allocate memory for linear image")?;
        unsafe { device.bind_image_memory(image_raw, device_memory, 0) }
            .context("Failed to bind device memory to linear image")?;
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
        let image_view = unsafe { device.create_image_view(&image_view_create_info, None) }
            .context("Failed to create ImageView for linear image")?;
        Ok(ManagedAndLinearImage {
            device,
            device_memory,
            image_raw,
            image_view,
        })
    }
}

impl Drop for ManagedAndLinearImage<'_> {
    fn drop(&mut self) {
        unsafe { self.device.destroy_image_view(self.image_view, None) };
        trace!("ImageView of linear image was destroyed");
        unsafe { self.device.destroy_image(self.image_raw, None) };
        trace!("Linear image was destroyed");
        unsafe { self.device.free_memory(self.device_memory, None) };
        trace!("GPU memory allocated for linear image was released");
    }
}
