use ash::{
    extensions::khr::{self, Surface},
    version::{EntryV1_0, InstanceV1_0},
    vk, Entry,
};
use std::ffi::{CStr, CString};

#[cfg(target_os = "windows")]
pub fn get_window_surface() -> &'static CStr {
    khr::Win32Surface::name()
}

#[cfg(target_os = "linux")]
pub fn get_window_surface() -> &'static CStr {
    khr::WaylandSurface::name()
}

pub struct Application {
    /// `ash::Instance` を利用するためには、同時に `ash::Entry` が Drop されずに存在している必要がある
    #[allow(dead_code)]
    entry: ash::Entry,
    instance: ash::Instance,
}

impl Application {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let entry = unsafe { Entry::new()? };
        let application_name = CString::new("Game")?;
        let application_name = &application_name.as_c_str();

        let engine_name = CString::new("No Engine")?;
        let engine_name = engine_name.as_c_str();

        let app_info = vk::ApplicationInfo::builder()
            .application_name(application_name)
            .application_version(vk::make_version(0, 1, 0))
            .engine_name(engine_name)
            .api_version(vk::make_version(1, 0, 0))
            .build();

        let extension_names = vec![Surface::name().as_ptr(), get_window_surface().as_ptr()];

        let create_info = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_extension_names(&extension_names);

        let instance = unsafe { entry.create_instance(&create_info, None)? };
        Ok(Application { entry, instance })
    }
}

impl Drop for Application {
    fn drop(&mut self) {
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}
