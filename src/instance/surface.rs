use ash::extensions::khr::{self, Surface};
use std::{ffi::CStr, os::raw::c_char};

pub fn get_surface_extensions() -> Vec<*const c_char> {
    vec![Surface::name().as_ptr(), get_window_surface().as_ptr()]
}

#[cfg(target_os = "windows")]
fn get_window_surface() -> &'static CStr {
    khr::Win32Surface::name()
}

#[cfg(target_os = "linux")]
fn get_window_surface() -> &'static CStr {
    khr::WaylandSurface::name()
}
