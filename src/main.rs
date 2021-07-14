extern crate game;

use ash::Entry;
use game::{glfw_wrapper::GlfwWrapper, instance::ManagedInstance};

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let entry = unsafe { Entry::new() }?;
    let glfw = GlfwWrapper::new()?;
    let instance = ManagedInstance::new(&entry, &glfw, cfg!(feature = "validation_layers"))?;
    let window = instance.create_window(500, 300, "Game")?;
    let logical_device = instance.create_logical_device(&window)?;
    let command_pool = logical_device.create_command_pool()?;
    command_pool.allocate_command_buffer()?;
    Ok(())
}
