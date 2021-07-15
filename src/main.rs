extern crate game;

use ash::Entry;
use game::{glfw_wrapper::GlfwWrapper, instance::ManagedInstance};

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let width: u32 = 500;
    let height: u32 = 300;
    let entry = unsafe { Entry::new() }?;
    let glfw = GlfwWrapper::new()?;
    let instance = ManagedInstance::new(&entry, &glfw, cfg!(feature = "validation_layers"))?;
    // let window = instance.create_window(width, height, "Game")?;
    // let logical_device = instance.create_logical_device(Some(&window))?;
    let logical_device = instance.create_logical_device(None)?;
    let command_pool = logical_device.create_command_pool()?;
    let graphics_queue = logical_device.get_graphics_queue();
    let command_buffer = command_pool.allocate_command_buffer()?;
    let image = logical_device.create_image(width, height)?;
    let render_pass = logical_device.create_render_pass()?;
    let _pipeline = render_pass.create_graphics_pipeline(width, height)?;
    let _framebuffer = logical_device.create_framebuffer(&render_pass, &image, width, height)?;
    command_buffer.submit_empty_cmd(&graphics_queue)?;
    Ok(())
}
