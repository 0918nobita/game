use ash::{
    extensions::khr::Surface,
    vk::{PhysicalDevice, SurfaceKHR},
};
use glfw::Window;

/// 自動で解放される、GLFW ウィンドウとそのサーフェスのラッパー
pub struct ManagedWindow {
    _window_raw: Window,
    surface_loader: Surface,
    surface: SurfaceKHR,
}

impl ManagedWindow {
    pub fn new(mut window_raw: Window, surface_loader: Surface, surface: SurfaceKHR) -> Self {
        window_raw.set_key_polling(true);

        ManagedWindow {
            _window_raw: window_raw,
            surface_loader,
            surface,
        }
    }

    pub fn get_physical_device_surface_support(
        &self,
        physical_device: &PhysicalDevice,
        queue_family_index: u32,
    ) -> bool {
        unsafe {
            self.surface_loader.get_physical_device_surface_support(
                *physical_device,
                queue_family_index,
                self.surface,
            )
        }
        .unwrap_or(false)
    }
}

impl Drop for ManagedWindow {
    fn drop(&mut self) {
        unsafe { self.surface_loader.destroy_surface(self.surface, None) };
        trace!("SurfaceKHR was destroyed");
    }
}
