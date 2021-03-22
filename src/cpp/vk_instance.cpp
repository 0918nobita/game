// Copyright 2021 Kodai Matsumoto

#include "vk_instance.hpp"

#include <GLFW/glfw3.h>

#include <iostream>

std::vector<const char *> getRequiredInstanceExtensions() noexcept {
    uint32_t num_required_exts;
    auto required_exts = glfwGetRequiredInstanceExtensions(&num_required_exts);
    std::cout << "Required extensions (" << num_required_exts << "):" << std::endl;
    std::vector<const char *> extensions(num_required_exts);
    for (uint32_t i = 0; i < num_required_exts; i++) {
        std::cout << "  " << required_exts[i] << std::endl;
        extensions[i] = required_exts[i];
    }
    return extensions;
}

vk::UniqueHandle<vk::Instance, vk::DispatchLoaderStatic> createInstance(
    const std::vector<const char *> &instance_exts, const std::vector<const char *> &layers) {
    const auto app_info = vk::ApplicationInfo("Application", VK_MAKE_VERSION(0, 1, 0));
    const auto instance_create_info =
        vk::InstanceCreateInfo()
            .setPApplicationInfo(&app_info)
            .setEnabledExtensionCount(static_cast<uint32_t>(instance_exts.size()))
            .setPpEnabledExtensionNames(instance_exts.data())
            .setPpEnabledLayerNames(layers.data());
    return vk::createInstanceUnique(instance_create_info);
}
