//! 開発中のインディーゲーム

#[macro_use]
extern crate log;
mod instance;
mod layer;
mod logical_device;
mod physical_device;
mod queue_family;

use ash::{
    version::{DeviceV1_0, InstanceV1_0},
    vk::{PhysicalDevice, Queue},
    Device, Entry, Instance,
};
use glfw::Context;

/// Vulkan インスタンスと、それがメモリ上に存在しているときに同時に存在している必要のあるデータをまとめたもの
pub struct Application {
    instance: Instance,
    logical_device: Device,
    /// `ash::Instance` を利用するためには、同時に `ash::Entry` が Drop されずに存在している必要がある
    _entry: Entry,
    _physical_device: PhysicalDevice,
    _graphics_queue: Queue,
}

impl Application {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        trace!("Initialization started");
        let entry = unsafe { Entry::new()? };
        let instance = instance::create_instance(&entry)?;
        let physical_device = physical_device::pick_physical_device(&instance);
        let (device, graphics_queue) =
            logical_device::create_logical_device_with_graphics_queue(&instance, physical_device);
        trace!("Initialization completed");
        Ok(Application {
            instance,
            logical_device: device,
            _entry: entry,
            _physical_device: physical_device,
            _graphics_queue: graphics_queue,
        })
    }

    pub fn run(&self) {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        let (mut window, event_receiver) = glfw
            .create_window(500, 300, "Game", glfw::WindowMode::Windowed)
            .expect("Failed to create window");

        window.set_key_polling(true);
        window.make_current();

        while !window.should_close() {
            glfw.poll_events();
            for (_, event) in glfw::flush_messages(&event_receiver) {
                match event {
                    glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _)
                    | glfw::WindowEvent::Close => window.set_should_close(true),
                    _ => (),
                }
            }
        }
    }
}

impl Drop for Application {
    fn drop(&mut self) {
        unsafe {
            self.logical_device.destroy_device(None);
            self.instance.destroy_instance(None);
        }
        trace!("Logical device and instance were released");
    }
}
