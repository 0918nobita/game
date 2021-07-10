use anyhow::Context;

pub struct GlfwWrapper {
    glfw_raw: glfw::Glfw,
}

impl GlfwWrapper {
    pub fn create_instance() -> anyhow::Result<Self> {
        let mut glfw_raw = glfw::init(glfw::FAIL_ON_ERRORS).context("Failed to initialize GLFW")?;
        glfw_raw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));
        ensure!(glfw_raw.vulkan_supported(), "Vulkan is not supported");
        Ok(GlfwWrapper { glfw_raw })
    }
}
