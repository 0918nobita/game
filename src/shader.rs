use anyhow::Context;
use ash::{
    util::read_spv,
    version::DeviceV1_0,
    vk::{PipelineShaderStageCreateInfo, ShaderModule, ShaderModuleCreateInfo, ShaderStageFlags},
    Device,
};
use once_cell::sync::Lazy;
use std::{
    ffi::{CStr, CString},
    io::Cursor,
};

static VERT_SPV: &[u8] = include_bytes!("../shaders/vert.spv");

static FRAG_SPV: &[u8] = include_bytes!("../shaders/frag.spv");

pub static VERT_SHADER: Lazy<(Vec<u32>, ShaderStageFlags)> = Lazy::new(|| {
    (
        read_spv(&mut Cursor::new(VERT_SPV)).unwrap(),
        ShaderStageFlags::VERTEX,
    )
});

pub static FRAG_SHADER: Lazy<(Vec<u32>, ShaderStageFlags)> = Lazy::new(|| {
    (
        read_spv(&mut Cursor::new(FRAG_SPV)).unwrap(),
        ShaderStageFlags::FRAGMENT,
    )
});

static STAGE_NAME: Lazy<CString> = Lazy::new(|| CString::new("main").unwrap());

pub struct ShaderModuleWrapper<'a> {
    logical_device: &'a Device,
    shader_module_raw: ShaderModule,
    shader_stage_flags: ShaderStageFlags,
}

impl<'a> ShaderModuleWrapper<'a> {
    pub fn new(
        logical_device: &'a Device,
        code: &[u32],
        shader_stage_flags: ShaderStageFlags,
    ) -> anyhow::Result<ShaderModuleWrapper<'a>> {
        let create_info = ShaderModuleCreateInfo::builder().code(code).build();
        let shader_module_raw = unsafe { logical_device.create_shader_module(&create_info, None) }
            .context("Failed to create shader module")?;
        trace!("Shader module {:?} was created", shader_module_raw);
        Ok(ShaderModuleWrapper {
            logical_device,
            shader_module_raw,
            shader_stage_flags,
        })
    }

    pub fn create_stage(&self) -> PipelineShaderStageCreateInfo {
        PipelineShaderStageCreateInfo::builder()
            .stage(self.shader_stage_flags)
            .module(self.shader_module_raw)
            .name(unsafe { CStr::from_ptr(STAGE_NAME.as_ptr()) })
            .build()
    }
}

impl<'a> Drop for ShaderModuleWrapper<'a> {
    fn drop(&mut self) {
        let shader_module_id = format!("{:?}", self.shader_module_raw);
        unsafe {
            self.logical_device
                .destroy_shader_module(self.shader_module_raw, None);
        }
        trace!("Shader module {} was destroyed", shader_module_id)
    }
}
