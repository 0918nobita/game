use anyhow::Context;
use ash::{
    util::read_spv,
    version::DeviceV1_0,
    vk::{ShaderModule, ShaderModuleCreateInfo},
    Device,
};
use once_cell::sync::Lazy;
use std::io::Cursor;

static VERT_SPV: &[u8] = include_bytes!("../shaders/vert.spv");

static FRAG_SPV: &[u8] = include_bytes!("../shaders/frag.spv");

pub static VERT_SHADER: Lazy<Vec<u32>> =
    Lazy::new(|| read_spv(&mut Cursor::new(VERT_SPV)).unwrap());

pub static FRAG_SHADER: Lazy<Vec<u32>> =
    Lazy::new(|| read_spv(&mut Cursor::new(FRAG_SPV)).unwrap());

pub struct ShaderModuleWrapper<'a> {
    logical_device: &'a Device,
    shader_module_raw: ShaderModule,
}

impl<'a> ShaderModuleWrapper<'a> {
    pub fn new(
        logical_device: &'a Device,
        code: &[u32],
    ) -> anyhow::Result<ShaderModuleWrapper<'a>> {
        let create_info = ShaderModuleCreateInfo::builder().code(code).build();
        let shader_module_raw = unsafe { logical_device.create_shader_module(&create_info, None) }
            .context("Failed to create shader module")?;
        trace!("Shader module {:?} was created", shader_module_raw);
        Ok(ShaderModuleWrapper {
            logical_device,
            shader_module_raw,
        })
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
