use std::ffi::CString;

use ash::{
    extensions::khr::{Surface, Win32Surface},
    version::{EntryV1_0, InstanceV1_0},
    vk, Entry,
};

fn main() {
    let entry = unsafe { Entry::new().expect("Failed to create entry") };

    let application_name = CString::new("Game").unwrap();
    let application_name = &application_name.as_c_str();

    let engine_name = CString::new("No Engine").unwrap();
    let engine_name = engine_name.as_c_str();

    let app_info = vk::ApplicationInfo::builder()
        .application_name(application_name)
        .application_version(vk::make_version(0, 1, 0))
        .engine_name(engine_name)
        .api_version(vk::make_version(1, 0, 0))
        .build();

    let extension_names = vec![Surface::name().as_ptr(), Win32Surface::name().as_ptr()];

    let create_info = vk::InstanceCreateInfo::builder()
        .application_info(&app_info)
        .enabled_extension_names(&extension_names);

    let instance = unsafe { entry.create_instance(&create_info, None).unwrap() };
    unsafe { instance.destroy_instance(None) }
}
