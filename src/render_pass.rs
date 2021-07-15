use anyhow::Context;
use ash::{
    version::DeviceV1_0,
    vk::{
        AttachmentDescription, AttachmentLoadOp, AttachmentReference, AttachmentStoreOp,
        ColorComponentFlags, CullModeFlags, Extent2D, Format, FrontFace,
        /*GraphicsPipelineCreateInfo, */ ImageLayout, Offset2D,
        /*Pipeline, */ PipelineBindPoint,
        /*PipelineCache, */ PipelineColorBlendAttachmentState,
        PipelineColorBlendStateCreateInfo, PipelineInputAssemblyStateCreateInfo,
        PipelineLayoutCreateInfo, PipelineMultisampleStateCreateInfo,
        PipelineRasterizationStateCreateInfo,
        /*PipelineShaderStageCreateInfo, */ PipelineVertexInputStateCreateInfo,
        PipelineViewportStateCreateInfo, PolygonMode, PrimitiveTopology, Rect2D, RenderPass,
        RenderPassCreateInfo, SampleCountFlags, /*ShaderStageFlags, */ SubpassDescription,
        Viewport,
    },
    Device,
};

use crate::shader::{ShaderModuleWrapper, FRAG_SHADER, VERT_SHADER};
// use once_cell::sync::Lazy;
// use std::ffi::{CStr, CString};

// static STAGE_NAME: Lazy<CString> = Lazy::new(|| CString::new("main").unwrap());

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

    pub fn create_graphics_pipeline(&self, width: u32, height: u32) -> anyhow::Result<()> {
        let viewport = Viewport {
            x: 0.0,
            y: 0.0,
            min_depth: 0.0,
            max_depth: 1.0,
            width: width as f32,
            height: height as f32,
        };
        let scissor = Rect2D {
            offset: Offset2D { x: 0, y: 0 },
            extent: Extent2D { width, height },
        };
        let _viewport_state = PipelineViewportStateCreateInfo::builder()
            .viewports(&[viewport])
            .scissors(&[scissor])
            .build();
        let _vertex_input_info = PipelineVertexInputStateCreateInfo::builder()
            .vertex_attribute_descriptions(&[])
            .vertex_binding_descriptions(&[])
            .build();
        let _input_assembly = PipelineInputAssemblyStateCreateInfo::builder()
            .topology(PrimitiveTopology::TRIANGLE_LIST)
            .primitive_restart_enable(false)
            .build();
        let _rasterizer = PipelineRasterizationStateCreateInfo::builder()
            .depth_clamp_enable(false)
            .rasterizer_discard_enable(false)
            .polygon_mode(PolygonMode::FILL)
            .line_width(1.0f32)
            .cull_mode(CullModeFlags::BACK)
            .front_face(FrontFace::CLOCKWISE)
            .depth_bias_enable(false)
            .build();
        let _multisample = PipelineMultisampleStateCreateInfo::builder()
            .sample_shading_enable(false)
            .rasterization_samples(SampleCountFlags::TYPE_1)
            .build();
        let blend_attachment = PipelineColorBlendAttachmentState::builder()
            .color_write_mask(
                ColorComponentFlags::R
                    | ColorComponentFlags::G
                    | ColorComponentFlags::B
                    | ColorComponentFlags::A,
            )
            .blend_enable(false)
            .build();
        let _blend = PipelineColorBlendStateCreateInfo::builder()
            .logic_op_enable(false)
            .attachments(&[blend_attachment])
            .build();
        let layout_create_info = PipelineLayoutCreateInfo::builder().set_layouts(&[]).build();
        let pipeline_layout = unsafe {
            self.device_raw
                .create_pipeline_layout(&layout_create_info, None)
        }?;
        let _vert_shader = ShaderModuleWrapper::new(&self.device_raw, &VERT_SHADER)?;
        let _frag_shader = ShaderModuleWrapper::new(&self.device_raw, &FRAG_SHADER)?;
        /*let create_info = GraphicsPipelineCreateInfo::builder()
            .viewport_state(&viewport_state)
            .vertex_input_state(&vertex_input_info)
            .input_assembly_state(&input_assembly)
            .rasterization_state(&rasterizer)
            .multisample_state(&multisample)
            .color_blend_state(&blend)
            .layout(pipeline_layout)
            .stages(&[PipelineShaderStageCreateInfo::builder()
                .stage(ShaderStageFlags::VERTEX)
                .name(unsafe { CStr::from_ptr(STAGE_NAME.as_ptr()) })
                .build()])
            .render_pass(self.render_pass_raw)
            .subpass(0)
            .build();
        if let Ok(pipelines) = unsafe {
            self.device_raw
                .create_graphics_pipelines(PipelineCache::null(), &[create_info], None)
        } {
            Ok(*(pipelines
                .first()
                .context("Failed to create graphics pipeline")?))
        } else {
            bail!("Failed to create graphics pipeline")
        }*/
        unsafe {
            self.device_raw
                .destroy_pipeline_layout(pipeline_layout, None);
            trace!("PipelineLayout was destroyed");
        };
        Ok(())
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
