//! Vulkan インスタンス関連

use super::layer;
use ash::{
    version::EntryV1_0,
    vk::{make_version, ApplicationInfo, InstanceCreateInfo},
    Entry, Instance,
};
use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};

pub fn create_instance(entry: &Entry, extension_names: &[String]) -> anyhow::Result<Instance> {
    let application_name = CString::new("Game")?;
    let application_name = &application_name.as_c_str();

    let engine_name = CString::new("No Engine")?;
    let engine_name = engine_name.as_c_str();

    let app_info = ApplicationInfo::builder()
        .application_name(application_name)
        .application_version(make_version(0, 1, 0))
        .engine_name(engine_name)
        .api_version(make_version(1, 0, 0))
        .build();

    let layer_names = if cfg!(feature = "validation_layers") {
        debug!("Validation layers: enabled");
        layer::check_validation_layer_support(&entry);
        layer::get_layer_name_ptrs()
    } else {
        debug!("Validation layers: disabled");
        Vec::new()
    };

    let extension_names: Vec<CString> = extension_names
        .iter()
        .map(|item| CString::new(item.as_str()).unwrap())
        .collect();
    let extension_names_raw: Vec<*const c_char> =
        extension_names.iter().map(|item| item.as_ptr()).collect();

    let create_info = InstanceCreateInfo::builder()
        .application_info(&app_info)
        .enabled_extension_names(&extension_names_raw)
        .enabled_layer_names(&layer_names);

    let instance = unsafe { entry.create_instance(&create_info, None)? };
    trace!(
        "Instance extensions: {}",
        entry
            .enumerate_instance_extension_properties()?
            .iter()
            .map(
                |ext_prop| unsafe { CStr::from_ptr(ext_prop.extension_name.as_ptr()) }
                    .to_str()
                    .unwrap()
            )
            .collect::<Vec<_>>()
            .join(", ")
    );

    Ok(instance)
}
