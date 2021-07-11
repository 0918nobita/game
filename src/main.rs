extern crate game;

use ash::Entry;
use game::{instance::ManagedInstance, managed_glfw::ManagedGlfw};

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let entry = unsafe { Entry::new() }?;
    let glfw = ManagedGlfw::new()?;
    let instance = ManagedInstance::new(
        &entry,
        &glfw.get_required_instance_extensions()?,
        cfg!(feature = "validation_layers"),
    )?;
    let _window = glfw.create_window(&instance, 500, 300, "Game")?;
    let _physical_device = instance.find_physical_device(|_device| true)?;
    Ok(())
}
