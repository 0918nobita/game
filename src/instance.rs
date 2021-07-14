//! Vulkan インスタンス関連

mod logical_device;
mod window;

use self::logical_device::ManagedLogicalDevice;
use crate::glfw_wrapper::GlfwWrapper;
use anyhow::Context;
use ash::{
    extensions::khr::{Surface, Swapchain},
    version::{EntryV1_0, InstanceV1_0},
    vk::{
        make_version, ApplicationInfo, DeviceCreateInfo, DeviceQueueCreateInfo, Handle,
        InstanceCreateInfo, PhysicalDevice, PhysicalDeviceFeatures, QueueFamilyProperties,
        QueueFlags, SurfaceKHR,
    },
    Entry, Instance,
};
use once_cell::sync::Lazy;
use std::ffi::CStr;
use std::{ffi::CString, os::raw::c_char};
use window::ManagedWindow;

/// 自動で解放される、Vulkan インスタンスのラッパー
pub struct ManagedInstance<'a> {
    entry: &'a Entry,
    glfw: &'a GlfwWrapper,
    instance_raw: Instance,
}

static VALIDATION_LAYERS: Lazy<Vec<CString>> =
    Lazy::new(|| vec![CString::new("VK_LAYER_KHRONOS_validation").unwrap()]);

impl<'a> ManagedInstance<'a> {
    pub fn new(
        entry: &'a Entry,
        glfw: &'a GlfwWrapper,
        with_validation_layers: bool,
    ) -> anyhow::Result<ManagedInstance<'a>> {
        let application_name = CString::new("Game")?;
        let engine_name = CString::new("No Engine")?;
        let app_info = ApplicationInfo::builder()
            .application_name(application_name.as_c_str())
            .application_version(make_version(0, 1, 0))
            .engine_name(engine_name.as_c_str())
            .build();

        let enabled_layer_names = if with_validation_layers {
            debug!("Validation layers: enabled");
            (*VALIDATION_LAYERS)
                .iter()
                .map(|name| name.as_ptr())
                .collect()
        } else {
            debug!("Validation layers: disabled");
            Vec::new()
        };

        let enabled_extension_names: Vec<CString> = glfw
            .get_required_instance_extensions()?
            .iter()
            .map(|item| CString::new(item.as_str()).unwrap())
            .collect();
        let enabled_extension_names: Vec<*const c_char> = enabled_extension_names
            .iter()
            .map(|item| item.as_ptr())
            .collect();

        let create_info = InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_extension_names(&enabled_extension_names)
            .enabled_layer_names(&enabled_layer_names)
            .build();
        let instance_raw = unsafe { entry.create_instance(&create_info, None) }
            .context("Failed to create Vulkan instance")?;

        Ok(ManagedInstance {
            entry,
            glfw,
            instance_raw,
        })
    }

    pub fn create_window<Title>(
        &self,
        width: u32,
        height: u32,
        title: Title,
    ) -> anyhow::Result<ManagedWindow>
    where
        Title: ToString,
    {
        let window_raw = self.glfw.create_window_raw(width, height, title)?;

        let surface_loader = Surface::new(self.entry, &self.instance_raw);

        let mut surface_raw = 0;
        window_raw.create_window_surface(
            self.instance_raw.handle().as_raw() as vk_sys::Instance,
            std::ptr::null(),
            &mut surface_raw,
        );
        let surface = SurfaceKHR::from_raw(surface_raw);

        Ok(ManagedWindow::new(window_raw, surface_loader, surface))
    }

    pub fn create_logical_device(
        &self,
        window: &ManagedWindow,
    ) -> anyhow::Result<ManagedLogicalDevice> {
        let (physical_device, queue_indices) =
            unsafe { self.instance_raw.enumerate_physical_devices() }
                .context("Failed to enumerate physical deviuces")?
                .into_iter()
                .find_map(|physical_device| {
                    try_get_queue_family_indices(physical_device, &self.instance_raw, window)
                })
                .context("No suitable physical device")?;

        let queue_create_infos = queue_indices
            .iter()
            .map(|index| {
                DeviceQueueCreateInfo::builder()
                    .queue_family_index(*index)
                    .queue_priorities(&[1.0f32])
                    .build()
            })
            .collect::<Vec<_>>();
        let device_features = PhysicalDeviceFeatures::builder().build();
        let layer_name_ptrs: Vec<*const c_char> = (*VALIDATION_LAYERS)
            .iter()
            .map(|name| name.as_ptr())
            .collect();
        let device_create_info = DeviceCreateInfo::builder()
            .queue_create_infos(&queue_create_infos)
            .enabled_extension_names(&[Swapchain::name().as_ptr()])
            .enabled_features(&device_features)
            .enabled_layer_names(&layer_name_ptrs)
            .build();
        let device_raw = unsafe {
            self.instance_raw
                .create_device(physical_device, &device_create_info, None)
        }
        .context("Failed to create logical device")?;
        Ok(ManagedLogicalDevice::new(device_raw, queue_indices))
    }
}

impl<'a> Drop for ManagedInstance<'a> {
    fn drop(&mut self) {
        unsafe { self.instance_raw.destroy_instance(None) }
        trace!("Vulkan instance was destroyed")
    }
}

fn try_get_queue_family_indices(
    physical_device: PhysicalDevice,
    instance_raw: &Instance,
    window: &ManagedWindow,
) -> Option<(PhysicalDevice, Vec<u32>)> {
    let queue_families =
        unsafe { instance_raw.get_physical_device_queue_family_properties(physical_device) };
    if !check_swapchain_support(instance_raw, &physical_device) {
        return None;
    }
    let graphics_queue_index = find_graphics_queue_family_index(&queue_families)?;
    let presentation_queue_index =
        find_presentation_queue_family_index(&queue_families, &physical_device, window)?;
    let mut queue_indices = vec![graphics_queue_index, presentation_queue_index];
    queue_indices.dedup();
    Some((physical_device, queue_indices))
}

fn find_graphics_queue_family_index(queue_families: &[QueueFamilyProperties]) -> Option<u32> {
    queue_families
        .iter()
        .enumerate()
        .find_map(|(queue_family_index, queue_family)| {
            queue_family
                .queue_flags
                .contains(QueueFlags::GRAPHICS)
                .then(|| queue_family_index as u32)
        })
}

fn check_swapchain_support(instance_raw: &Instance, physical_device: &PhysicalDevice) -> bool {
    unsafe { instance_raw.enumerate_device_extension_properties(*physical_device) }
        .map(|exts| {
            exts.into_iter().any(
                |ext| unsafe { CStr::from_ptr(ext.extension_name.as_ptr()) } == Swapchain::name(),
            )
        })
        .unwrap_or(false)
}

fn find_presentation_queue_family_index(
    queue_families: &[QueueFamilyProperties],
    physical_device: &PhysicalDevice,
    window: &ManagedWindow,
) -> Option<u32> {
    queue_families
        .iter()
        .enumerate()
        .find_map(|(queue_family_index, _)| {
            let queue_family_index = queue_family_index as u32;
            window
                .get_physical_device_surface_support(&physical_device, queue_family_index)
                .then(|| queue_family_index)
        })
}
