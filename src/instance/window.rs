use ash::{extensions::khr::Surface, vk::SurfaceKHR};
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
}

impl Drop for ManagedWindow {
    fn drop(&mut self) {
        unsafe { self.surface_loader.destroy_surface(self.surface, None) };
        trace!("SurfaceKHR was destroyed");
    }
}
