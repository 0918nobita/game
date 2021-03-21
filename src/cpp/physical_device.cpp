// Copyright 2021 Kodai Matsumoto

#include "physical_device.hpp"

#include <GLFW/glfw3.h>

#include <iostream>

std::string physicalDeviceTypeToStr(vk::PhysicalDeviceType device_type) noexcept {
    switch (device_type) {
        case vk::PhysicalDeviceType::eCpu:
            return "CPU";
        case vk::PhysicalDeviceType::eDiscreteGpu:
            return "Discrete GPU";
        case vk::PhysicalDeviceType::eIntegratedGpu:
            return "Integrated GPU";
        case vk::PhysicalDeviceType::eVirtualGpu:
            return "Virtual GPU";
        default:
            return "Other GPU";
    }
}

void dumpPhysicalDevices(const std::vector<vk::PhysicalDevice> &devices) noexcept {
    std::cout << "Physical devices (" << devices.size() << "):" << std::endl;
    for (const auto &device : devices) {
        const auto props = device.getProperties();
        std::cout << "  " << props.deviceName << " (" << physicalDeviceTypeToStr(props.deviceType)
                  << ")" << std::endl;
    }
}

void selectPhysicalDeviceAndQueueFamily(const vk::Instance &instance,
                                        const std::vector<vk::PhysicalDevice> &devices,
                                        vk::PhysicalDevice *selected_device,
                                        uint32_t *queue_family_index, uint32_t *queue_count) {
    for (const auto &device : devices) {
        const auto queue_family_props = device.getQueueFamilyProperties();
        for (uint32_t i = 0; i < queue_family_props.size(); i++) {
            const auto prop = queue_family_props[i];
            if ((uint32_t)prop.queueFlags &&
                glfwGetPhysicalDevicePresentationSupport(instance, device, i)) {
                *queue_family_index = i;
                *queue_count = prop.queueCount;
                *selected_device = device;
                return;
            }
        }
    }

    std::cerr << "No device supports image presentation to window surface" << std::endl;
    std::exit(EXIT_FAILURE);
}

void dumpDeviceExtensions(const vk::PhysicalDevice &physical_device) {
    const auto device_extension_props = physical_device.enumerateDeviceExtensionProperties();
    std::cout << "Provided device extensions (" << device_extension_props.size()
              << "):" << std::endl;
    for (const auto &prop : device_extension_props)
        std::cout << "  " << prop.extensionName << std::endl;
}
