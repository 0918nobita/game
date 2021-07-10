extern crate game;

use ash::Entry;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let entry = unsafe { Entry::new() }?;
    let glfw = game::managed_glfw::ManagedGlfw::new()?;
    let instance =
        game::instance::ManagedInstance::new(&entry, &glfw.get_required_instance_extensions()?)?;
    let _window = glfw.create_window(&instance, 500, 300, "Game")?;
    Ok(())
}
