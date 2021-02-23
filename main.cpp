// Copyright 2021 Kodai Matsumoto

#define GLFW_INCLUDE_VULKAN
#include <GLFW/glfw3.h>
#include <iostream>
#include <vulkan/vulkan.hpp>

int main() {
    glfwInit();

    uint32_t num_required_exts;
    auto required_ext = glfwGetRequiredInstanceExtensions(&num_required_exts);
    std::vector<const char *> extensions(num_required_exts);
    std::cout << "Required extensions (" << num_required_exts << "):" << std::endl;
    for (uint32_t i = 0u; i < num_required_exts; i++) {
        std::cout << '\t' << required_ext[i] << std::endl;
        extensions[i] = required_ext[i];
    }

    std::vector<const char *> layers = { "VK_LAYER_LUNARG_standard_validation" };

    const auto app_info = vk::ApplicationInfo("Application", VK_MAKE_VERSION(0, 1, 0));
    auto instance =
        vk::createInstanceUnique(
            vk::InstanceCreateInfo()
                .setPApplicationInfo(&app_info)
                .setEnabledExtensionCount(extensions.size())
                .setPpEnabledExtensionNames(extensions.data())
                .setPpEnabledLayerNames(layers.data()));

    auto devices = instance->enumeratePhysicalDevices();
    if (devices.empty()) {
        std::cerr << "No physical device available for Vulkan" << std::endl;
        return EXIT_FAILURE;
    }
    std::cout << "Physical devices (" << devices.size() << "):" << std::endl;
    for (const auto &device : devices) {
        const auto props = device.getProperties();
        std::cout << '\t' << props.deviceName << " (";
        switch (props.deviceType) {
            case vk::PhysicalDeviceType::eCpu:
                std::cout << "CPU";
                break;
            case vk::PhysicalDeviceType::eDiscreteGpu:
                std::cout << "Discrete GPU";
                break;
            case vk::PhysicalDeviceType::eIntegratedGpu:
                std::cout << "Integrated GPU";
                break;
            case vk::PhysicalDeviceType::eVirtualGpu:
                std::cout << "Virtual GPU";
                break;
            case vk::PhysicalDeviceType::eOther:
                std::cout << "Other GPU";
                break;
        }
        std::cout << ")" << std::endl;
    }

    glfwWindowHint(GLFW_CLIENT_API, GLFW_NO_API);

    GLFWwindow *window = glfwCreateWindow(600, 500, "Application", nullptr, nullptr);

    VkSurfaceKHR surface;
    const auto result = glfwCreateWindowSurface(instance.get(), window, nullptr, &surface);
    if (result != VK_SUCCESS) {
        std::cerr << "Failed to create window surface" << std::endl;
        return EXIT_FAILURE;
    }

    while (!glfwWindowShouldClose(window)) {
        glfwPollEvents();
    }

    glfwDestroyWindow(window);
    glfwTerminate();

    return EXIT_SUCCESS;
}
