//! GLFW 関連

use anyhow::Context;
use glfw::{ClientApiHint, Glfw, Window, WindowHint, WindowMode};

pub struct GlfwWrapper {
    glfw_raw: Glfw,
}

impl GlfwWrapper {
    pub fn new() -> anyhow::Result<Self> {
        let mut glfw_raw = glfw::init(glfw::FAIL_ON_ERRORS).context("Failed to initialize GLFW")?;
        glfw_raw.window_hint(WindowHint::ClientApi(ClientApiHint::NoApi));
        ensure!(glfw_raw.vulkan_supported(), "Vulkan is not supported");
        Ok(GlfwWrapper { glfw_raw })
    }

    pub fn create_window_raw<'a, Title>(
        &'a self,
        width: u32,
        height: u32,
        title: Title,
    ) -> anyhow::Result<Window>
    where
        Title: ToString,
    {
        let (window, _event_receiver) = self
            .glfw_raw
            .create_window(width, height, &title.to_string(), WindowMode::Windowed)
            .context("Failed to create window")?;
        Ok(window)
    }

    pub fn get_required_instance_extensions(&self) -> anyhow::Result<Vec<String>> {
        self.glfw_raw
            .get_required_instance_extensions()
            .context("Failed to get required instance extensions")
    }
}
