use ash::{
    version::{DeviceV1_0, InstanceV1_0},
    vk::Handle,
    Entry,
};

#[macro_use]
extern crate log;
#[macro_use(defer)]
extern crate scopeguard;
extern crate game;

use game::{logical_device, physical_device};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    trace!("Initialization started");

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));
    assert!(glfw.vulkan_supported());

    let required_extensions = glfw.get_required_instance_extensions().unwrap();
    debug!(
        "Instance extensions required by GLFW: {:?}",
        required_extensions
    );

    let entry = unsafe { Entry::new()? };

    let instance = game::instance::create_instance(&entry, &required_extensions)?;
    defer! {
        unsafe { instance.destroy_instance(None) }
        trace!("Instance was destroyed")
    }

    let physical_device = physical_device::pick_physical_device(&instance);

    let (logical_device, _graphics_queue) =
        logical_device::create_logical_device_with_graphics_queue(&instance, physical_device);
    defer! {
        unsafe { logical_device.destroy_device(None) }
        trace!("Logical device was destroyed");
    }

    let (mut window, event_receiver) = glfw
        .create_window(500, 300, "Game", glfw::WindowMode::Windowed)
        .expect("Failed to create window");

    window.set_key_polling(true);

    let mut raw_surface = 0;
    let result = window.create_window_surface(
        instance.handle().as_raw() as vk_sys::Instance,
        std::ptr::null(),
        &mut raw_surface,
    );
    assert!(result == 0, "Failed to create window surface");

    let surface_loader = ash::extensions::khr::Surface::new(&entry, &instance);
    let surface = ash::vk::SurfaceKHR::from_raw(raw_surface);
    defer! {
        unsafe { surface_loader.destroy_surface(surface, None) }
        trace!("SurfaceKHR was destroyed")
    }

    trace!("Event loop started");
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
