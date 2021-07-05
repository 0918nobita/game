#[macro_use]
extern crate log;
mod surface;

use ash::{
    version::{DeviceV1_0, EntryV1_0, InstanceV1_0},
    vk::{
        make_version, ApplicationInfo, DeviceCreateInfo, DeviceQueueCreateInfo, InstanceCreateInfo,
        PhysicalDevice, PhysicalDeviceFeatures, Queue, QueueFamilyProperties, QueueFlags,
    },
    Device, Entry, Instance,
};
use once_cell::sync::Lazy;
use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};

static VALIDATION_LAYERS: Lazy<Vec<CString>> =
    Lazy::new(|| vec![CString::new("VK_LAYER_KHRONOS_validation").unwrap()]);

/// Vulkan インスタンスと、それがメモリ上に存在しているときに同時に存在している必要のあるデータをまとめたもの
pub struct Application {
    instance: Instance,
    logical_device: Device,
    /// `ash::Instance` を利用するためには、同時に `ash::Entry` が Drop されずに存在している必要がある
    _entry: Entry,
    _physical_device: PhysicalDevice,
    _graphics_queue: Queue,
}

impl Application {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        trace!("Initialization started");
        let entry = unsafe { Entry::new()? };
        let instance = create_instance(&entry)?;
        let physical_device = pick_physical_device(&instance);
        let (device, graphics_queue) =
            create_logical_device_with_graphics_queue(&instance, physical_device);
        trace!("Initialization completed");
        Ok(Application {
            instance,
            logical_device: device,
            _entry: entry,
            _physical_device: physical_device,
            _graphics_queue: graphics_queue,
        })
    }
}

impl Drop for Application {
    fn drop(&mut self) {
        unsafe {
            self.logical_device.destroy_device(None);
            self.instance.destroy_instance(None);
        }
        trace!("Logical device and instance were released");
    }
}

fn create_instance(entry: &Entry) -> Result<Instance, Box<dyn std::error::Error>> {
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

    check_validation_layer_support(&entry);

    let layer_names = get_layer_name_ptrs();

    let create_info = InstanceCreateInfo::builder()
        .application_info(&app_info)
        .enabled_extension_names(&extension_names)
        .enabled_layer_names(&layer_names);

    Ok(unsafe { entry.create_instance(&create_info, None)? })
}

fn check_validation_layer_support(entry: &Entry) {
    assert!(
        (*VALIDATION_LAYERS).iter().all(|layer_name| {
            entry
                .enumerate_instance_layer_properties()
                .unwrap()
                .iter()
                .any(|layer| {
                    let name = unsafe { CStr::from_ptr(layer.layer_name.as_ptr()).to_owned() };
                    name == *layer_name
                })
        }),
        "Some validation layer not supported"
    )
}

fn pick_physical_device(instance: &Instance) -> PhysicalDevice {
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

fn create_logical_device_with_graphics_queue(
    instance: &Instance,
    device: PhysicalDevice,
) -> (Device, Queue) {
    let queue_family_index = find_queue_families(instance, &device).unwrap();
    let queue_priorities = [1.0f32];
    let queue_create_infos = [DeviceQueueCreateInfo::builder()
        .queue_family_index(queue_family_index)
        .queue_priorities(&queue_priorities)
        .build()];
    let device_features = PhysicalDeviceFeatures::builder().build();

    let layer_name_ptrs = get_layer_name_ptrs();

    let device_create_info = DeviceCreateInfo::builder()
        .queue_create_infos(&queue_create_infos)
        .enabled_features(&device_features)
        .enabled_layer_names(&layer_name_ptrs)
        .build();

    let device = unsafe {
        instance
            .create_device(device, &device_create_info, None)
            .unwrap()
    };
    let graphics_queue = unsafe { device.get_device_queue(queue_family_index, 0) };
    (device, graphics_queue)
}

fn get_layer_name_ptrs() -> Vec<*const c_char> {
    (*VALIDATION_LAYERS)
        .iter()
        .map(|name| name.as_ptr())
        .collect()
}

fn find_queue_families(instance: &Instance, device: &PhysicalDevice) -> Option<u32> {
    let props = unsafe { instance.get_physical_device_queue_family_properties(*device) };
    props
        .iter()
        .enumerate()
        .find(|(_, family)| is_queue_family_suitable(family))
        .map(|(index, _)| index as _)
}

fn is_device_suitable(instance: &Instance, device: &PhysicalDevice) -> bool {
    let props = unsafe { instance.get_physical_device_queue_family_properties(*device) };

    props.iter().any(is_queue_family_suitable)
}

fn is_queue_family_suitable(family: &QueueFamilyProperties) -> bool {
    family.queue_count > 0 && family.queue_flags.contains(QueueFlags::GRAPHICS)
}
