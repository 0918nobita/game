use crate::window::ManagedWindow;
use anyhow::Context;

pub struct ManagedGlfw {
    glfw_raw: glfw::Glfw,
}

impl ManagedGlfw {
    pub fn new() -> anyhow::Result<Self> {
        let mut glfw_raw = glfw::init(glfw::FAIL_ON_ERRORS).context("Failed to initialize GLFW")?;
        glfw_raw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));
        ensure!(glfw_raw.vulkan_supported(), "Vulkan is not supported");
        Ok(ManagedGlfw { glfw_raw })
    }

    pub fn create_window<'a, Title>(
        &'a self,
        instance: &'a crate::instance::ManagedInstance,
        width: u32,
        height: u32,
        title: Title,
    ) -> anyhow::Result<ManagedWindow>
    where
        Title: ToString,
    {
        let (window, _event_receiver) = self
            .glfw_raw
            .create_window(
                width,
                height,
                &title.to_string(),
                glfw::WindowMode::Windowed,
            )
            .context("Failed to create window")?;
        Ok(ManagedWindow::new(window, instance))
    }

    pub fn get_required_instance_extensions(&self) -> anyhow::Result<Vec<String>> {
        self.glfw_raw
            .get_required_instance_extensions()
            .context("Failed to get required instance extensions")
    }
}
