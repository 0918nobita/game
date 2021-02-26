// Copyright 2021 Kodai Matsumoto

#define GLFW_INCLUDE_VULKAN
#include <GLFW/glfw3.h>

#include <iostream>
#include <vulkan/vulkan.hpp>

void prepareGLFW();
std::vector<const char *> get_required_instance_extensions();
std::string physical_device_type_to_str(vk::PhysicalDeviceType device_type);
void main_loop(GLFWwindow *window);
void cleanup(GLFWwindow *window);

int main() {
    prepareGLFW();

    auto instance_exts = get_required_instance_extensions();

    std::vector<const char *> layers = {"VK_LAYER_LUNARG_standard_validation"};
    const auto app_info = vk::ApplicationInfo("Application", VK_MAKE_VERSION(0, 1, 0));
    auto instance = vk::createInstanceUnique(vk::InstanceCreateInfo()
                                                 .setPApplicationInfo(&app_info)
                                                 .setEnabledExtensionCount(instance_exts.size())
                                                 .setPpEnabledExtensionNames(instance_exts.data())
                                                 .setPpEnabledLayerNames(layers.data()));

    auto devices = instance->enumeratePhysicalDevices();
    if (devices.empty()) {
        std::cerr << "No physical device available for Vulkan" << std::endl;
        return EXIT_FAILURE;
    }
    std::cout << "\nPhysical devices (" << devices.size() << "):" << std::endl;
    for (const auto &device : devices) {
        const auto props = device.getProperties();
        std::cout << "  " << props.deviceName << " ("
                  << physical_device_type_to_str(props.deviceType) << ")" << std::endl;
        const auto queue_family_props = device.getQueueFamilyProperties();
        std::cout << "    Queue Families (" << queue_family_props.size() << "):" << std::endl;
        for (const auto &prop : queue_family_props) {
            std::cout << "      Queue family found (queue count: " << prop.queueCount << ")";
            if ((uint32_t)prop.queueFlags & VK_QUEUE_GRAPHICS_BIT) {
                std::cout << " <- for graphics";
            }
            std::cout << std::endl;
        }
    }

    GLFWwindow *window = glfwCreateWindow(600, 500, "Application", nullptr, nullptr);

    VkSurfaceKHR surface;
    const auto result = glfwCreateWindowSurface(instance.get(), window, nullptr, &surface);
    if (result != VK_SUCCESS) {
        std::cerr << "Failed to create window surface" << std::endl;
        return EXIT_FAILURE;
    }

    main_loop(window);
    cleanup(window);
    return EXIT_SUCCESS;
}

/** GLFW の初期設定を済ませる */
void prepareGLFW() {
    glfwInit();
    glfwWindowHint(GLFW_CLIENT_API, GLFW_NO_API);
    glfwWindowHint(GLFW_RESIZABLE, 0);
}

/** GLFW が必要としているインスタンス拡張を一括取得する */
std::vector<const char *> get_required_instance_extensions() {
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

std::string physical_device_type_to_str(vk::PhysicalDeviceType device_type) {
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

void main_loop(GLFWwindow *window) {
    while (!glfwWindowShouldClose(window)) {
        glfwPollEvents();
    }
}

void cleanup(GLFWwindow *window) {
    glfwDestroyWindow(window);
    glfwTerminate();
}
