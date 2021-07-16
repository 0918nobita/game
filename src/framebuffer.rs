use crate::{optimized_image::ManagedAndOptimizedImage, render_pass::ManagedRenderPass};
use ash::{
    version::DeviceV1_0,
    vk::{Framebuffer, FramebufferCreateInfo},
    Device,
};

pub struct ManagedFramebuffer<'a> {
    device: &'a Device,
    _render_pass: &'a ManagedRenderPass<'a>,
    _connectable_image: &'a ManagedAndOptimizedImage<'a>,
    framebuffer_raw: Framebuffer,
}

impl<'a> ManagedFramebuffer<'a> {
    pub fn new(
        device: &'a Device,
        render_pass: &'a ManagedRenderPass<'a>,
        connectable_image: &'a ManagedAndOptimizedImage<'a>,
        width: u32,
        height: u32,
    ) -> anyhow::Result<ManagedFramebuffer<'a>> {
        let create_info = FramebufferCreateInfo::builder()
            .width(width)
            .height(height)
            .layers(1)
            .render_pass(render_pass.get_render_pass_raw())
            .attachments(&[connectable_image.get_image_view_raw()])
            .build();
        let framebuffer_raw = unsafe { device.create_framebuffer(&create_info, None) }?;
        Ok(ManagedFramebuffer {
            device,
            _render_pass: render_pass,
            _connectable_image: connectable_image,
            framebuffer_raw,
        })
    }

    pub fn get_framebuffer_raw(&self) -> Framebuffer {
        self.framebuffer_raw
    }
}

impl Drop for ManagedFramebuffer<'_> {
    fn drop(&mut self) {
        unsafe { self.device.destroy_framebuffer(self.framebuffer_raw, None) };
        trace!("Framebuffer was destroyed");
    }
}
