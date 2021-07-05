//! Vulkan インスタンス関連

mod surface;

use super::layer;
use ash::{
    version::EntryV1_0,
    vk::{make_version, ApplicationInfo, InstanceCreateInfo},
    Entry, Instance,
};
use std::ffi::CString;

pub fn create_instance(entry: &Entry) -> Result<Instance, Box<dyn std::error::Error>> {
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

    let extension_names = surface::get_surface_extensions();

    let layer_names = if cfg!(feature = "validation_layers") {
        debug!("Validation layers will be enabled");
        layer::check_validation_layer_support(&entry);
        layer::get_layer_name_ptrs()
    } else {
        Vec::new()
    };

    let create_info = InstanceCreateInfo::builder()
        .application_info(&app_info)
        .enabled_extension_names(&extension_names)
        .enabled_layer_names(&layer_names);

    Ok(unsafe { entry.create_instance(&create_info, None)? })
}
