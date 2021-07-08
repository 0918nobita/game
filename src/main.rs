use ash::{
    extensions::khr::Surface,
    version::{DeviceV1_0, InstanceV1_0},
    vk::{Handle, SharingMode, SurfaceKHR},
    Entry,
};
use game::{device, swapchain::SwapchainWrapper, window};
use thiserror::Error;

#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate log;
#[macro_use]
extern crate scopeguard;
extern crate game;

#[derive(Error, Debug)]
enum InitializationError {
    #[error("GLFW doesn't support Vulkan")]
    VulkanNotSupported,
    #[error("Failed to create window surface")]
    WindowSurfaceCreationFailed,
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    trace!("Initialization started");

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));
    ensure!(
        glfw.vulkan_supported(),
        InitializationError::VulkanNotSupported
    );

    let required_extensions = glfw.get_required_instance_extensions().unwrap();
    trace!(
        "Instance extensions required by GLFW: {}",
        required_extensions.join(", ")
    );

    let entry = unsafe { Entry::new()? };

    let instance = game::instance::create_instance(&entry, &required_extensions)?;
    defer! {
        unsafe { instance.destroy_instance(None) }
        trace!("Instance was destroyed")
    }

    let (mut window, event_receiver) = glfw
        .create_window(
            window::WIDTH,
            window::HEIGHT,
            "Game",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create window");
    window.set_key_polling(true);
    trace!("Window was created");

    let mut raw_surface = 0;
    let result = window.create_window_surface(
        instance.handle().as_raw() as vk_sys::Instance,
        std::ptr::null(),
        &mut raw_surface,
    );
    ensure!(
        result == 0,
        InitializationError::WindowSurfaceCreationFailed
    );

    let surface = Surface::new(&entry, &instance);
    let surface_khr = SurfaceKHR::from_raw(raw_surface);
    defer! {
        unsafe { surface.destroy_surface(surface_khr, None) }
        trace!("SurfaceKHR was destroyed")
    }

    let (physical_device, logical_device, queues) =
        device::create_device_and_queue_indices(&instance, &surface, surface_khr)?;
    defer! {
        unsafe { logical_device.destroy_device(None) }
        trace!("Logical device was destroyed")
    }

    let _graphics_queue = unsafe { logical_device.get_device_queue(queues.graphics, 0) };
    let _present_queue = unsafe { logical_device.get_device_queue(queues.presentation, 0) };

    let mut queue_family_indices = vec![queues.graphics, queues.presentation];
    queue_family_indices.dedup();
    let image_sharing_mode = if queue_family_indices.len() > 1 {
        SharingMode::CONCURRENT
    } else {
        SharingMode::EXCLUSIVE
    };
    debug!("Image sharing mode: {:?}", image_sharing_mode);

    let _swapchain = SwapchainWrapper::new(
        &instance,
        &surface,
        surface_khr,
        &physical_device,
        &logical_device,
        image_sharing_mode,
        &queue_family_indices,
    )?;

    trace!("Event loop was started");
    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&event_receiver) {
            match event {
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _)
                | glfw::WindowEvent::Close => {
                    window.set_should_close(true);
                    trace!("Window closing");
                }
                _ => (),
            }
        }
    }
    trace!("Event loop was ended");

    Ok(())
}
