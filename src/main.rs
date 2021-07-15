extern crate game;

use ash::Entry;
use game::{glfw_wrapper::GlfwWrapper, instance::ManagedInstance};

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let entry = unsafe { Entry::new() }?;
    let glfw = GlfwWrapper::new()?;
    let instance = ManagedInstance::new(&entry, &glfw, cfg!(feature = "validation_layers"))?;
    // let window = instance.create_window(500, 300, "Game")?;
    // let logical_device = instance.create_logical_device(Some(&window))?;
    let logical_device = instance.create_logical_device(None)?;
    let command_pool = logical_device.create_command_pool()?;
    let graphics_queue = logical_device.get_graphics_queue();
    let command_buffer = command_pool.allocate_command_buffer()?;
    let _image = logical_device.create_image(500, 300)?;
    let render_pass = logical_device.create_render_pass()?;
    render_pass.create_graphics_pipeline(500, 300)?;
    command_buffer.submit_empty_cmd(&graphics_queue)?;
    Ok(())
}
