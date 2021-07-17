#[macro_use]
extern crate log;

use anyhow::Context;
use ash::version::{EntryV1_0, InstanceV1_0};
use once_cell::sync::Lazy;
use std::ffi::CString;

static APPLICATION_NAME: Lazy<CString> = Lazy::new(|| CString::new("Hello Triangle").unwrap());
static ENGINE_NAME: Lazy<CString> = Lazy::new(|| CString::new("No Engine").unwrap());
static VALIDATION_LAYERS: Lazy<Vec<CString>> = Lazy::new(|| {
    if cfg!(feature = "validation_layers") {
        vec![CString::new("VK_LAYER_KHRONOS_validation").unwrap()]
    } else {
        vec![]
    }
});

pub struct PhysicalDevice {
    device_type: ash::vk::PhysicalDeviceType,
    device_name: String,
}

impl std::fmt::Debug for PhysicalDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({:?})", self.device_name, self.device_type)
    }
}

pub struct Instance {
    raw: ash::Instance,
}

impl Instance {
    pub fn new(entry: &ash::Entry) -> anyhow::Result<Self> {
        let app_info = ash::vk::ApplicationInfo::builder()
            .application_name(APPLICATION_NAME.as_c_str())
            .application_version(ash::vk::make_version(0, 1, 0))
            .engine_name(ENGINE_NAME.as_c_str())
            .build();
        let enabled_layer_names = (*VALIDATION_LAYERS)
            .iter()
            .map(|name| name.as_ptr())
            .collect::<Vec<_>>();
        let instance_create_info = ash::vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_extension_names(&[])
            .enabled_layer_names(&enabled_layer_names)
            .build();
        let raw = unsafe { entry.create_instance(&instance_create_info, None) }
            .context("Failed to create Vulkan instance")?;
        trace!("[CREATED] Vulkan instance");
        Ok(Instance { raw })
    }

    pub fn enumerate_physical_devices(&self) -> anyhow::Result<Vec<PhysicalDevice>> {
        unsafe { self.raw.enumerate_physical_devices() }
            .context("Failed to enumerate physical devices")
            .map(|raw_physical_devices| {
                raw_physical_devices
                    .into_iter()
                    .map(|raw_physical_device| {
                        wrap_raw_physical_device(&self.raw, raw_physical_device)
                    })
                    .collect::<Vec<_>>()
            })
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe { self.raw.destroy_instance(None) }
        trace!("[DESTROYED] Vulkan instance")
    }
}

fn wrap_raw_physical_device(
    raw_instance: &ash::Instance,
    raw_physical_device: ash::vk::PhysicalDevice,
) -> PhysicalDevice {
    let props = unsafe { raw_instance.get_physical_device_properties(raw_physical_device) };
    PhysicalDevice {
        device_name: { string_from_i8_array(props.device_name) },
        device_type: props.device_type,
    }
}

fn string_from_i8_array(arr: [i8; 256]) -> String {
    let device_name: &[u8] =
        unsafe { std::intrinsics::transmute(std::slice::from_raw_parts(arr.as_ptr(), 256)) };
    let device_name = device_name.to_vec();
    String::from_utf8(device_name).unwrap()
}
