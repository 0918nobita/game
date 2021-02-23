// Copyright 2021 Kodai Matsumoto

#define GLFW_INCLUDE_VULKAN
#include <GLFW/glfw3.h>
#include <iostream>
#include <vulkan/vulkan.hpp>

std::string physicalDeviceTypeToStr(vk::PhysicalDeviceType deviceType);
void mainLoop(GLFWwindow* window);
void cleanup(GLFWwindow* window);

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
        std::cout << '\t' << props.deviceName << " (" << physicalDeviceTypeToStr(props.deviceType) << ")" << std::endl;
    }

    glfwWindowHint(GLFW_CLIENT_API, GLFW_NO_API);

    GLFWwindow *window = glfwCreateWindow(600, 500, "Application", nullptr, nullptr);

    VkSurfaceKHR surface;
    const auto result = glfwCreateWindowSurface(instance.get(), window, nullptr, &surface);
    if (result != VK_SUCCESS) {
        std::cerr << "Failed to create window surface" << std::endl;
        return EXIT_FAILURE;
    }

    mainLoop(window);
    cleanup(window);
    return EXIT_SUCCESS;
}

std::string physicalDeviceTypeToStr(vk::PhysicalDeviceType deviceType) {
    switch (deviceType) {
        case vk::PhysicalDeviceType::eCpu:
            return "CPU";
        case vk::PhysicalDeviceType::eDiscreteGpu:
            return "Discrete GPU";
        case vk::PhysicalDeviceType::eIntegratedGpu:
            return "Integrated GPU";
        case vk::PhysicalDeviceType::eVirtualGpu:
            return "Virtual GPU";
        case vk::PhysicalDeviceType::eOther:
            return "Other GPU";
    }
}

void mainLoop(GLFWwindow* window) {
    while (!glfwWindowShouldClose(window)) {
        glfwPollEvents();
    }
}

void cleanup(GLFWwindow* window) {
    glfwDestroyWindow(window);
    glfwTerminate();
}
