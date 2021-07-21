extern crate game;
#[macro_use]
extern crate log;

use anyhow::Context;
use game::instance::Instance;
use std::rc::Rc;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let entry = unsafe { ash::Entry::new() }?;
    let instance = Instance::new(&entry)?;
    let physical_devices = instance.enumerate_physical_devices()?;
    physical_devices
        .iter()
        .for_each(|physical_device| debug!("PhysicalDevice: {:?}", physical_device));
    let physical_device = physical_devices
        .first()
        .context("No suitable physical device")?;
    let logical_device = instance.create_logical_device(physical_device)?;
    let _command_pool =
        Rc::clone(&logical_device).create_command_pool(&physical_device.graphics_queue_family())?;
    trace!("Complete.");
    Ok(())
}
