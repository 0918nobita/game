use crate::queue_family_index::{Graphics, QueueFamilyIndex};
use std::marker::PhantomData;

pub struct PhysicalDevice {
    raw: ash::vk::PhysicalDevice,
    device_type: ash::vk::PhysicalDeviceType,
    device_name: String,
    graphics_queue_family: QueueFamilyIndex<Graphics>,
}

impl PhysicalDevice {
    pub fn builder() -> PhysicalDeviceBuilder<No, No, No, No> {
        PhysicalDeviceBuilder::default()
    }

    pub fn raw(&self) -> &ash::vk::PhysicalDevice {
        &self.raw
    }

    pub fn graphics_queue_family(&self) -> &QueueFamilyIndex<Graphics> {
        &self.graphics_queue_family
    }
}

impl std::fmt::Debug for PhysicalDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} (DeviceType: {:?}, GraphicsQueueFamily: {:?})",
            self.device_name, self.device_type, self.graphics_queue_family
        )
    }
}

#[derive(Default)]
pub struct Yes;
#[derive(Default)]
pub struct No;

#[derive(Default)]
pub struct PhysicalDeviceBuilder<RawSet, DeviceTypeSet, DeviceNameSet, GraphicsQueueFamilySet> {
    raw: ash::vk::PhysicalDevice,
    _raw_set: PhantomData<RawSet>,

    device_type: ash::vk::PhysicalDeviceType,
    _device_type_set: PhantomData<DeviceTypeSet>,

    device_name: String,
    _device_name_set: PhantomData<DeviceNameSet>,

    graphics_queue_family: QueueFamilyIndex<Graphics>,
    _graphics_queue_family_set: PhantomData<GraphicsQueueFamilySet>,
}

impl PhysicalDeviceBuilder<Yes, Yes, Yes, Yes> {
    pub fn build(self) -> PhysicalDevice {
        PhysicalDevice {
            raw: self.raw,
            device_type: self.device_type,
            device_name: self.device_name,
            graphics_queue_family: self.graphics_queue_family,
        }
    }
}

impl<R, DT, DN, G> PhysicalDeviceBuilder<R, DT, DN, G> {
    pub fn raw(self, raw: ash::vk::PhysicalDevice) -> PhysicalDeviceBuilder<Yes, DT, DN, G> {
        PhysicalDeviceBuilder {
            raw,
            _raw_set: PhantomData,
            device_type: self.device_type,
            _device_type_set: self._device_type_set,
            device_name: self.device_name,
            _device_name_set: self._device_name_set,
            graphics_queue_family: self.graphics_queue_family,
            _graphics_queue_family_set: self._graphics_queue_family_set,
        }
    }

    pub fn device_type(
        self,
        device_type: ash::vk::PhysicalDeviceType,
    ) -> PhysicalDeviceBuilder<R, Yes, DN, G> {
        PhysicalDeviceBuilder {
            raw: self.raw,
            _raw_set: self._raw_set,
            device_type,
            _device_type_set: PhantomData,
            device_name: self.device_name,
            _device_name_set: self._device_name_set,
            graphics_queue_family: self.graphics_queue_family,
            _graphics_queue_family_set: self._graphics_queue_family_set,
        }
    }

    pub fn device_name(self, device_name: String) -> PhysicalDeviceBuilder<R, DT, Yes, G> {
        PhysicalDeviceBuilder {
            raw: self.raw,
            _raw_set: self._raw_set,
            device_type: self.device_type,
            _device_type_set: self._device_type_set,
            device_name,
            _device_name_set: PhantomData,
            graphics_queue_family: self.graphics_queue_family,
            _graphics_queue_family_set: self._graphics_queue_family_set,
        }
    }

    pub fn graphics_queue_family(
        self,
        graphics_queue_family: QueueFamilyIndex<Graphics>,
    ) -> PhysicalDeviceBuilder<R, DT, DN, Yes> {
        PhysicalDeviceBuilder {
            raw: self.raw,
            _raw_set: self._raw_set,
            device_type: self.device_type,
            _device_type_set: self._device_type_set,
            device_name: self.device_name,
            _device_name_set: self._device_name_set,
            graphics_queue_family,
            _graphics_queue_family_set: PhantomData,
        }
    }
}
