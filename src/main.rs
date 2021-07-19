extern crate game;
#[macro_use]
extern crate log;

use game::instance::Instance;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let entry = unsafe { ash::Entry::new() }?;
    let instance = Instance::new(&entry)?;
    let physical_devices = instance.enumerate_physical_devices()?;
    physical_devices
        .iter()
        .for_each(|physical_device| debug!("PhysicalDevice: {:?}", physical_device));
    trace!("Complete.");
    Ok(())
}
