use std::cell::RefCell;

use ash::{version::DeviceV1_0, vk, Device};

use crate::{
    shader::{ShaderModuleWrapper, FRAG_SHADER, VERT_SHADER},
    swapchain::SwapchainWrapper,
};

pub fn create_pipeline(device: &Device, swapchain: &SwapchainWrapper) -> anyhow::Result<()> {
    let _vertex_shader_module = ShaderModuleWrapper::new(device, &VERT_SHADER)?;
    let _fragment_shader_module = ShaderModuleWrapper::new(&device, &FRAG_SHADER)?;

    // シェーダモジュールはシェーダの外から出入りするデータに binding という番号を付けている
    // バッファやイメージはそれ自体には通し番号を持っていないため、binding とバッファやイメージの対応関係を示すデスクリプタセットが必要
    // シェーダモジュールとデスクリプタセットが組み合わさることで、シェーダがアクセスしようとしているデータが具体的にどこに置かれたどんなデータなのかが確定する

    let _descriptor_set_layout_bindings = [ash::vk::DescriptorSetLayoutBinding::builder()
        .descriptor_type(ash::vk::DescriptorType::UNIFORM_BUFFER)
        .descriptor_count(1)
        .binding(0)
        .stage_flags(ash::vk::ShaderStageFlags::VERTEX)
        .build()];
    let swapchain_image_count = swapchain.get_image_count()?;
    debug!("Swapchain image count: {}", swapchain_image_count);
    let descriptor_set_layout = RefCell::new(Vec::<ash::vk::DescriptorSetLayout>::with_capacity(
        swapchain_image_count,
    ));
    defer! {
        for item in descriptor_set_layout.borrow().iter() {
            unsafe { device.destroy_descriptor_set_layout(*item, None)}
        }
        trace!("All DescriptorSetLayout were destroyed");
    }

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
