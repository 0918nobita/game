use super::queue_family;
use ash::{version::InstanceV1_0, vk::PhysicalDevice, Instance};
use std::ffi::CStr;

pub fn pick_physical_device(instance: &Instance) -> PhysicalDevice {
    let devices = unsafe { instance.enumerate_physical_devices().unwrap() };

    let device = devices
        .into_iter()
        .find(|device| is_device_suitable(instance, device))
        .expect("No suitable physical device");

    let props = unsafe { instance.get_physical_device_properties(device) };
    let device_name = unsafe { CStr::from_ptr(props.device_name.as_ptr()) };
    let device_name = device_name.to_str().unwrap().to_owned();
    debug!("Selected physical device: {}", device_name);

    device
}

fn is_device_suitable(instance: &Instance, device: &PhysicalDevice) -> bool {
    let props = unsafe { instance.get_physical_device_queue_family_properties(*device) };

    props.iter().any(queue_family::is_queue_family_suitable)
}
