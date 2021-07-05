use ash::{
    extensions::khr::{self, Surface},
    version::{EntryV1_0, InstanceV1_0},
    vk, Entry, Instance,
};
use once_cell::sync::Lazy;
use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};

static VALIDATION_LAYERS: Lazy<Vec<String>> =
    Lazy::new(|| vec!["VK_LAYER_KHRONOS_validation".to_owned()]);

/// Vulkan インスタンスと、それがメモリ上に存在しているときに同時に存在している必要のあるデータをまとめたもの
pub struct Application {
    instance: Instance,
    /// `ash::Instance` を利用するためには、同時に `ash::Entry` が Drop されずに存在している必要がある
    _entry: Entry,
    _physical_device: vk::PhysicalDevice,
}

impl Application {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let entry = unsafe { Entry::new()? };
        let instance = create_instance(&entry)?;
        let physical_device = pick_physical_device(&instance);
        Ok(Application {
            instance,
            _entry: entry,
            _physical_device: physical_device,
        })
    }
}

impl Drop for Application {
    fn drop(&mut self) {
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}

fn create_instance(entry: &Entry) -> Result<Instance, Box<dyn std::error::Error>> {
    let application_name = CString::new("Game")?;
    let application_name = &application_name.as_c_str();

    let engine_name = CString::new("No Engine")?;
    let engine_name = engine_name.as_c_str();

    let app_info = vk::ApplicationInfo::builder()
        .application_name(application_name)
        .application_version(vk::make_version(0, 1, 0))
        .engine_name(engine_name)
        .api_version(vk::make_version(1, 0, 0))
        .build();

    let extension_names = vec![Surface::name().as_ptr(), get_window_surface().as_ptr()];

    check_validation_layer_support(&entry);

    let layer_names: Vec<CString> = (*VALIDATION_LAYERS)
        .iter()
        .map(|layer| CString::new(layer.clone()).unwrap())
        .collect();
    let layer_names: Vec<*const c_char> = layer_names.iter().map(|name| name.as_ptr()).collect();

    let create_info = vk::InstanceCreateInfo::builder()
        .application_info(&app_info)
        .enabled_extension_names(&extension_names)
        .enabled_layer_names(&layer_names);

    Ok(unsafe { entry.create_instance(&create_info, None)? })
}

#[cfg(target_os = "windows")]
fn get_window_surface() -> &'static CStr {
    khr::Win32Surface::name()
}

#[cfg(target_os = "linux")]
fn get_window_surface() -> &'static CStr {
    khr::WaylandSurface::name()
}

fn check_validation_layer_support(entry: &Entry) {
    assert!(
        (*VALIDATION_LAYERS).iter().all(|layer_name| {
            entry
                .enumerate_instance_layer_properties()
                .unwrap()
                .iter()
                .any(|layer| {
                    let name = unsafe { CStr::from_ptr(layer.layer_name.as_ptr()) };
                    let name = name.to_str().unwrap();
                    name == layer_name
                })
        }),
        "Some validation layer not supported"
    )
}

fn pick_physical_device(instance: &Instance) -> vk::PhysicalDevice {
    let devices = unsafe { instance.enumerate_physical_devices().unwrap() };

    let device = devices
        .into_iter()
        .find(|device| is_device_suitable(instance, device))
        .expect("No suitable physical device");

    let props = unsafe { instance.get_physical_device_properties(device) };
    let device_name = unsafe { CStr::from_ptr(props.device_name.as_ptr()) };
    let device_name = device_name.to_str().unwrap().to_owned();
    println!("Selected physical device: {}", device_name);

    device
}

fn is_device_suitable(instance: &Instance, device: &vk::PhysicalDevice) -> bool {
    let props = unsafe { instance.get_physical_device_queue_family_properties(*device) };

    props.iter().any(|family| {
        family.queue_count > 0 && family.queue_flags.contains(vk::QueueFlags::GRAPHICS)
    })
}
