use ash::{
    version::DeviceV1_0,
    vk::{
        ClearColorValue, ClearValue, CommandBuffer, CommandBufferBeginInfo, CommandPool, Extent2D,
        Fence, Offset2D, PipelineBindPoint, Queue, Rect2D, RenderPassBeginInfo, SubmitInfo,
        SubpassContents,
    },
    Device,
};
use crate::{
    framebuffer::ManagedFramebuffer, pipeline::ManagedPipeline, render_pass::ManagedRenderPass,
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
