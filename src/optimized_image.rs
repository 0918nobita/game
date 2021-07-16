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

pub struct ManagedAndOptimizedImage<'a> {
    device: &'a Device,
    device_memory: DeviceMemory,
    image_raw: Image,
    image_view: ImageView,
}

impl<'a> ManagedAndOptimizedImage<'a> {
    pub fn new(
        instance: &Instance,
        physical_device: &PhysicalDevice,
        device: &'a Device,
        width: u32,
        height: u32,
    ) -> anyhow::Result<ManagedAndOptimizedImage<'a>> {
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
            .tiling(ImageTiling::OPTIMAL)
            .initial_layout(ImageLayout::UNDEFINED)
            .usage(ImageUsageFlags::COLOR_ATTACHMENT | ImageUsageFlags::TRANSFER_SRC)
            .sharing_mode(SharingMode::EXCLUSIVE)
            .samples(SampleCountFlags::TYPE_1)
            .build();
        // 物理デバイスが持っているメモリについての情報
        let memory_properties =
            unsafe { instance.get_physical_device_memory_properties(*physical_device) };
        let image_raw = unsafe { device.create_image(&create_info, None) }
            .context("Failed to create optimized image")?;
        // イメージに対してどんな種類のメモリがどれくらいのサイズ必要か
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
        .context("Failed to allocate memory for optimized image")?;
        unsafe { device.bind_image_memory(image_raw, device_memory, 0) }
            .context("Failed to bind device memory to optimized image")?;
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
            .context("Failed to create ImageView for optimized image")?;
        Ok(ManagedAndOptimizedImage {
            device,
            device_memory,
            image_raw,
            image_view,
        })
    }

    pub fn get_image_raw(&self) -> Image {
        self.image_raw
    }

    pub fn get_image_view_raw(&self) -> ImageView {
        self.image_view
    }

    /*
    pub fn export_bitmap(&self, width: u32, height: u32) -> anyhow::Result<()> {
        let memory_requirements = unsafe {
            self.device_raw
                .get_image_memory_requirements(self.image_raw)
        };
        let mapped_memory = unsafe {
            self.device_raw.map_memory(
                self.device_memory_raw,
                0u64,
                memory_requirements.size,
                MemoryMapFlags::empty(),
            )
        }
        .context("Failed to map memory")? as *mut u8;
        let size =
            usize::try_from(memory_requirements.size).context("Failed to convert u64 to usize")?;
        debug!("Image size: {}", size);
        let mapped_memory = unsafe { from_raw_parts(mapped_memory, size) };
        let input_size = (width * height * 4) as usize;
        let mut input = vec![0u8; input_size];
        input.copy_from_slice(mapped_memory);
        let mut output = vec![0u8; (width * height * 3) as usize];
        for (output, chunk) in output.chunks_exact_mut(3).zip(input.chunks_exact(4)) {
            output.copy_from_slice(&chunk[0..3])
        }
        let image_buffer = image::ImageBuffer::<Rgb<u8>, _>::from_raw(width, height, output)
            .context("Failed to create image::ImageBuffer")?;
        image_buffer
            .save("triangle.bmp")
            .context("Failed to save image")?;
        unsafe { self.device_raw.unmap_memory(self.device_memory_raw) };
        Ok(())
    }
    */
}

impl Drop for ManagedAndOptimizedImage<'_> {
    fn drop(&mut self) {
        unsafe { self.device.destroy_image_view(self.image_view, None) };
        trace!("ImageView of optimized image was destroyed");
        unsafe { self.device.destroy_image(self.image_raw, None) };
        trace!("Optimized image was destroyed");
        unsafe { self.device.free_memory(self.device_memory, None) };
        trace!("GPU memory allocated for optimized image was released");
    }
}
