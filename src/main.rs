extern crate game;
#[macro_use]
extern crate log;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let entry = unsafe { ash::Entry::new() }?;
    let instance = game::Instance::new(&entry)?;
    let physical_devices = instance.enumerate_physical_devices()?;
    physical_devices
        .iter()
        .for_each(|physical_device| debug!("PhysicalDevice: {:?}", physical_device));
    debug!("Complete!");
    Ok(())
}
