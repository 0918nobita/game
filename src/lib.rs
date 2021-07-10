//! 開発中のインディーゲーム

#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate log;

mod glfw_wrapper;
mod physical_device;

use once_cell::sync::Lazy;

pub static GLFW: Lazy<glfw_wrapper::GlfwWrapper> = Lazy::new(|| {
    let glfw_wrapper = glfw_wrapper::GlfwWrapper::create_instance().expect("Failed to setup GLFW");
    trace!("GLFW setup succeeded");
    glfw_wrapper
});
