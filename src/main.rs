use ash::{
    version::{DeviceV1_0, InstanceV1_0},
    Entry,
};
use glfw::Context;

#[macro_use]
extern crate log;
#[macro_use(defer)]
extern crate scopeguard;
extern crate game;

use game::{logical_device, physical_device};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    trace!("Initialization started");

    let entry = unsafe { Entry::new()? };

    let instance = game::instance::create_instance(&entry)?;
    defer! {
        unsafe { instance.destroy_instance(None) }
        trace!("Instance was released")
    }

    let physical_device = physical_device::pick_physical_device(&instance);

    let (logical_device, _graphics_queue) =
        logical_device::create_logical_device_with_graphics_queue(&instance, physical_device);
    defer! {
        unsafe { logical_device.destroy_device(None) }
        trace!("Logical device was released");
    }

    trace!("Initialization completed");

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, event_receiver) = glfw
        .create_window(500, 300, "Game", glfw::WindowMode::Windowed)
        .expect("Failed to create window");

    window.set_key_polling(true);
    window.make_current();

    assert!(glfw.vulkan_supported());

    let required_extensions = glfw.get_required_instance_extensions().unwrap();
    debug!(
        "Instance extensions required by GLFW: {:?}",
        required_extensions
    );

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
    Ok(())
}
