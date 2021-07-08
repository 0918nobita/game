use ash::Device;

use crate::shader::{ShaderModuleWrapper, FRAG_SHADER, VERT_SHADER};

pub fn create_pipeline(device: &Device) -> anyhow::Result<()> {
    let _vertex_shader_module = ShaderModuleWrapper::new(device, &VERT_SHADER)?;
    let _fragment_shader_module = ShaderModuleWrapper::new(&device, &FRAG_SHADER)?;
    Ok(())
}
