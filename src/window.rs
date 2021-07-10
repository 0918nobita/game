use ash::{
    extensions::khr::Surface,
    vk::{Handle, SurfaceKHR},
};

pub struct ManagedWindow {
    _window_raw: glfw::Window,
    surface_loader: Surface,
    surface: SurfaceKHR,
}

impl ManagedWindow {
    pub fn new(mut window_raw: glfw::Window, instance: &crate::instance::ManagedInstance) -> Self {
        window_raw.set_key_polling(true);

        let surface_loader = instance.create_surface();

        let mut surface_raw = 0;
        window_raw.create_window_surface(
            instance.get_raw_vk_instance(),
            std::ptr::null(),
            &mut surface_raw,
        );
        let surface = ash::vk::SurfaceKHR::from_raw(surface_raw);

        ManagedWindow {
            _window_raw: window_raw,
            surface_loader,
            surface,
        }
    }
}

impl<'a> Drop for ManagedWindow {
    fn drop(&mut self) {
        unsafe { self.surface_loader.destroy_surface(self.surface, None) };
        trace!("SurfaceKHR was destroyed");
    }
}
