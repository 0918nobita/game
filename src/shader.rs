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

pub fn create_shader_module(device: &Device, code: &[u32]) -> anyhow::Result<ShaderModule> {
    let create_info = ShaderModuleCreateInfo::builder().code(code).build();
    unsafe { device.create_shader_module(&create_info, None) }
        .context("Failed to create shader module")
}
