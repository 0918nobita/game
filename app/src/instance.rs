use crate::logical_device::LogicalDevice;
use crate::physical_device::PhysicalDevice;
use crate::queue_family_index::{Graphics, QueueFamilyIndex};
use anyhow::Context;
use ash::version::{EntryV1_0, InstanceV1_0};
use once_cell::sync::Lazy;
use std::{ffi::CString, os::raw::c_char, rc::Rc};

static APPLICATION_NAME: Lazy<CString> = Lazy::new(|| CString::new("Hello Triangle").unwrap());
static ENGINE_NAME: Lazy<CString> = Lazy::new(|| CString::new("No Engine").unwrap());
static VALIDATION_LAYERS: Lazy<Vec<CString>> = Lazy::new(|| {
    if cfg!(feature = "validation_layers") {
        vec![CString::new("VK_LAYER_KHRONOS_validation").unwrap()]
    } else {
        vec![]
    }
});

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
                    .filter_map(|raw_physical_device| {
                        try_create_physical_device_with_graphics_queue(
                            &self.raw,
                            raw_physical_device,
                        )
                    })
                    .collect::<Vec<_>>()
            })
    }

    pub fn create_logical_device(
        &self,
        physical_device: &PhysicalDevice,
    ) -> anyhow::Result<Rc<LogicalDevice>> {
        let queue_create_info = ash::vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(**physical_device.graphics_queue_family())
            .queue_priorities(&[1.0f32])
            .build();
        let device_features = ash::vk::PhysicalDeviceFeatures::builder().build();
        let layer_name_ptrs: Vec<*const c_char> = (*VALIDATION_LAYERS)
            .iter()
            .map(|name| name.as_ptr())
            .collect();
        let device_create_info = ash::vk::DeviceCreateInfo::builder()
            .queue_create_infos(&[queue_create_info])
            .enabled_extension_names(&[])
            .enabled_features(&device_features)
            .enabled_layer_names(&layer_name_ptrs)
            .build();
        let device_raw = unsafe {
            self.raw
                .create_device(*physical_device.raw(), &device_create_info, None)
        }
        .context("Failed to create logical device")?;
        Ok(LogicalDevice::new(device_raw))
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe { self.raw.destroy_instance(None) }
        trace!("[DESTROYED] Vulkan instance")
    }
}

fn try_create_physical_device_with_graphics_queue(
    raw_instance: &ash::Instance,
    raw_physical_device: ash::vk::PhysicalDevice,
) -> Option<PhysicalDevice> {
    let props = unsafe { raw_instance.get_physical_device_properties(raw_physical_device) };
    let queue_families =
        unsafe { raw_instance.get_physical_device_queue_family_properties(raw_physical_device) };
    queue_families
        .iter()
        .enumerate()
        .find_map(|(queue_family_index, queue_family)| {
            queue_family
                .queue_flags
                .contains(ash::vk::QueueFlags::GRAPHICS)
                .then(|| QueueFamilyIndex::<Graphics>::new(queue_family_index as u32))
        })
        .map(|graphics_queue_family| {
            PhysicalDevice::builder()
                .raw(raw_physical_device)
                .device_type(props.device_type)
                .device_name(string_from_i8_array(props.device_name))
                .graphics_queue_family(graphics_queue_family)
                .build()
        })
}

fn string_from_i8_array(arr: [i8; 256]) -> String {
    let device_name: &[u8] =
        unsafe { std::intrinsics::transmute(std::slice::from_raw_parts(arr.as_ptr(), 256)) };
    let device_name = device_name.to_vec();
    String::from_utf8(device_name).unwrap()
}
