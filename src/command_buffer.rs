use crate::{
    framebuffer::ManagedFramebuffer, linear_image::ManagedAndLinearImage,
    optimized_image::ManagedAndOptimizedImage, pipeline::ManagedPipeline,
    render_pass::ManagedRenderPass,
};
use ash::{
    version::DeviceV1_0,
    vk::{
        ClearColorValue, ClearValue, CommandBuffer, CommandBufferBeginInfo,
        CommandBufferResetFlags, CommandPool, Extent2D, Fence, Filter, ImageAspectFlags, ImageBlit,
        ImageLayout, ImageSubresourceLayers, Offset2D, Offset3D, PipelineBindPoint, Queue, Rect2D,
        RenderPassBeginInfo, SubmitInfo, SubpassContents,
    },
    Device,
};

pub struct ManagedCommandBuffer<'a> {
    device: &'a Device,
    command_pool: &'a CommandPool,
    command_buffer_raw: CommandBuffer,
}

impl<'a> ManagedCommandBuffer<'a> {
    pub fn new(
        device: &'a Device,
        command_pool: &'a CommandPool,
        command_buffer_raw: CommandBuffer,
    ) -> ManagedCommandBuffer<'a> {
        ManagedCommandBuffer {
            device,
            command_pool,
            command_buffer_raw,
        }
    }

    pub fn draw_triangle(
        &self,
        queue: &Queue,
        render_pass: &ManagedRenderPass,
        framebuffer: &ManagedFramebuffer,
        pipeline: &ManagedPipeline,
        width: u32,
        height: u32,
    ) -> anyhow::Result<()> {
        let begin_info = CommandBufferBeginInfo::builder().build();
        let submit_info = SubmitInfo::builder()
            .command_buffers(&[self.command_buffer_raw])
            .build();
        unsafe {
            self.device
                .begin_command_buffer(self.command_buffer_raw, &begin_info)
        }?;
        let render_pass_begin_info = RenderPassBeginInfo::builder()
            .render_pass(render_pass.get_render_pass_raw())
            .framebuffer(framebuffer.get_framebuffer_raw())
            .render_area(
                Rect2D::builder()
                    .offset(Offset2D { x: 0, y: 0 })
                    .extent(Extent2D { width, height })
                    .build(),
            )
            .clear_values(&[ClearValue {
                color: ClearColorValue {
                    float32: [0.0f32, 0.0f32, 0.0f32, 1.0f32],
                },
            }])
            .build();
        unsafe {
            self.device.cmd_begin_render_pass(
                self.command_buffer_raw,
                &render_pass_begin_info,
                SubpassContents::INLINE,
            );
            self.device.cmd_bind_pipeline(
                self.command_buffer_raw,
                PipelineBindPoint::GRAPHICS,
                pipeline.get_pipeline_raw(),
            );
            self.device.cmd_draw(self.command_buffer_raw, 3, 1, 0, 0);
            self.device.cmd_end_render_pass(self.command_buffer_raw);
            self.device.end_command_buffer(self.command_buffer_raw)?;
            self.device
                .queue_submit(*queue, &[submit_info], Fence::null())?;
            self.device.queue_wait_idle(*queue)?;
        }
        Ok(())
    }

    pub fn blit_image(
        &self,
        queue: &Queue,
        src_image: &ManagedAndOptimizedImage,
        dst_image: &ManagedAndLinearImage,
        width: i32,
        height: i32,
    ) -> anyhow::Result<()> {
        let begin_info = CommandBufferBeginInfo::builder().build();
        let submit_info = SubmitInfo::builder()
            .command_buffers(&[self.command_buffer_raw])
            .build();
        unsafe {
            self.device.reset_command_buffer(
                self.command_buffer_raw,
                CommandBufferResetFlags::RELEASE_RESOURCES,
            )?;
            self.device
                .begin_command_buffer(self.command_buffer_raw, &begin_info)?;
            let blit_size = Offset3D {
                x: width,
                y: height,
                z: 1,
            };
            self.device.cmd_blit_image(
                self.command_buffer_raw,
                src_image.get_image_raw(),
                ImageLayout::GENERAL,
                dst_image.get_image_raw(),
                ImageLayout::GENERAL,
                &[ImageBlit::builder()
                    .src_offsets([Default::default(), blit_size])
                    .src_subresource(
                        ImageSubresourceLayers::builder()
                            .mip_level(0)
                            .base_array_layer(0)
                            .aspect_mask(ImageAspectFlags::COLOR)
                            .layer_count(1)
                            .build(),
                    )
                    .dst_offsets([Default::default(), blit_size])
                    .dst_subresource(
                        ImageSubresourceLayers::builder()
                            .mip_level(0)
                            .base_array_layer(0)
                            .aspect_mask(ImageAspectFlags::COLOR)
                            .layer_count(1)
                            .build(),
                    )
                    .build()],
                Filter::LINEAR,
            );
            self.device.end_command_buffer(self.command_buffer_raw)?;
            self.device
                .queue_submit(*queue, &[submit_info], Fence::null())?;
            self.device.queue_wait_idle(*queue)?;
        };
        Ok(())
    }
}

impl Drop for ManagedCommandBuffer<'_> {
    fn drop(&mut self) {
        unsafe {
            self.device
                .free_command_buffers(*self.command_pool, &[self.command_buffer_raw])
        }
        trace!("CommandBuffer was destroyed")
    }
}
