use std::{ffi::CString, os::raw::c_char};

use anyhow::Context;
use ash::{
    extensions::khr::Surface,
    version::{EntryV1_0, InstanceV1_0},
    vk::{self, Handle},
    Entry, Instance,
};
use once_cell::sync::Lazy;

pub struct ManagedInstance<'a> {
    entry: &'a Entry,
    instance_raw: Instance,
}

static VALIDATION_LAYERS: Lazy<Vec<CString>> =
    Lazy::new(|| vec![CString::new("VK_LAYER_KHRONOS_validation").unwrap()]);

impl<'a> ManagedInstance<'a> {
    pub fn new(
        entry: &'a Entry,
        extension_names: &[String],
    ) -> anyhow::Result<ManagedInstance<'a>> {
        let application_name = CString::new("Game")?;
        let engine_name = CString::new("No Engine")?;
        let app_info = vk::ApplicationInfo::builder()
            .application_name(application_name.as_c_str())
            .application_version(vk::make_version(0, 1, 0))
            .engine_name(engine_name.as_c_str())
            .build();
        let enabled_layer_names = if cfg!(feature = "validation_layers") {
            debug!("Validation layers: enabled");
            (*VALIDATION_LAYERS)
                .iter()
                .map(|name| name.as_ptr())
                .collect()
        } else {
            debug!("Validation layers: disabled");
            Vec::new()
        };
        let enabled_extension_names: Vec<CString> = extension_names
            .iter()
            .map(|item| CString::new(item.as_str()).unwrap())
            .collect();
        let enabled_extension_names: Vec<*const c_char> = enabled_extension_names
            .iter()
            .map(|item| item.as_ptr())
            .collect();
        let create_info = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_extension_names(&enabled_extension_names)
            .enabled_layer_names(&enabled_layer_names)
            .build();
        let instance_raw = unsafe { entry.create_instance(&create_info, None) }
            .context("Failed to create Vulkan instance")?;
        Ok(ManagedInstance {
            entry,
            instance_raw,
        })
    }

    pub fn create_surface(&self) -> Surface {
        Surface::new(self.entry, &self.instance_raw)
    }

    pub fn get_raw_vk_instance(&self) -> vk_sys::Instance {
        self.instance_raw.handle().as_raw() as vk_sys::Instance
    }
}

impl<'a> Drop for ManagedInstance<'a> {
    fn drop(&mut self) {
        unsafe { self.instance_raw.destroy_instance(None) }
        trace!("Vulkan instance was destroyed")
    }
}
