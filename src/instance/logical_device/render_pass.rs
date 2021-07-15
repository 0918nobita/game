use anyhow::Context;
use ash::{
    version::DeviceV1_0,
    vk::{
        AttachmentDescription, AttachmentLoadOp, AttachmentReference, AttachmentStoreOp, Format,
        ImageLayout, PipelineBindPoint, RenderPass, RenderPassCreateInfo, SampleCountFlags,
        SubpassDescription,
    },
    Device,
};

pub struct ManagedRenderPass<'a> {
    device_raw: &'a Device,
    render_pass_raw: RenderPass,
}

impl<'a> ManagedRenderPass<'a> {
    pub fn new(device_raw: &'a Device) -> anyhow::Result<ManagedRenderPass<'a>> {
        let attachment_desc = AttachmentDescription::builder()
            .format(Format::R8G8B8A8_UNORM)
            .samples(SampleCountFlags::TYPE_1)
            .load_op(AttachmentLoadOp::DONT_CARE)
            .store_op(AttachmentStoreOp::STORE)
            .stencil_load_op(AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(AttachmentStoreOp::DONT_CARE)
            .initial_layout(ImageLayout::UNDEFINED)
            .final_layout(ImageLayout::GENERAL)
            .build();
        let attachment_ref = AttachmentReference::builder()
            .attachment(0)
            .layout(ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .build();
        let subpass = SubpassDescription::builder()
            .pipeline_bind_point(PipelineBindPoint::GRAPHICS)
            .color_attachments(&[attachment_ref])
            .build();
        let create_info = RenderPassCreateInfo::builder()
            .attachments(&[attachment_desc])
            .subpasses(&[subpass])
            .dependencies(&[])
            .build();
        let render_pass_raw = unsafe { device_raw.create_render_pass(&create_info, None) }
            .context("Failed to create RenderPass")?;
        Ok(ManagedRenderPass {
            device_raw,
            render_pass_raw,
        })
    }
}

impl Drop for ManagedRenderPass<'_> {
    fn drop(&mut self) {
        unsafe {
            self.device_raw
                .destroy_render_pass(self.render_pass_raw, None)
        };
        trace!("RenderPass was destroyed");
    }
}
