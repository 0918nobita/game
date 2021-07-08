use ash::{vk, Device};

use crate::shader::{ShaderModuleWrapper, FRAG_SHADER, VERT_SHADER};

pub fn create_pipeline(device: &Device) -> anyhow::Result<()> {
    let _vertex_shader_module = ShaderModuleWrapper::new(device, &VERT_SHADER)?;
    let _fragment_shader_module = ShaderModuleWrapper::new(&device, &FRAG_SHADER)?;

    // ビューポートサイズからブレンド機能まで、グラフィックスパイプラインのすべての固定機能を明示的に設定する必要がある

    // 頂点シェーダに渡される頂点データの形式を指定する
    let _vertex_input_create_info = vk::PipelineVertexInputStateCreateInfo::builder().build();
    // 頂点からどのようなジオメトリが描画されるかを指定する
    let _input_assembly_create_info = vk::PipelineInputAssemblyStateCreateInfo::builder()
        // 再利用なしで3頂点から三角形を作る
        .topology(vk::PrimitiveTopology::TRIANGLE_LIST)
        // 頂点の再利用等の最適化をしない
        .primitive_restart_enable(false)
        .build();

    Ok(())
}
