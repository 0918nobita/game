// Copyright 2021 Kodai Matsumoto

#define GLFW_INCLUDE_VULKAN
#include <GLFW/glfw3.h>

#include <algorithm>
#include <iostream>
#include <vulkan/vulkan.hpp>

void prepareGLFW();

std::vector<const char *> getRequiredInstanceExtensions() noexcept;
vk::UniqueHandle<vk::Instance, vk::DispatchLoaderStatic> createInstance(
    const std::vector<const char *> &instance_exts, const std::vector<const char *> &layers);
void dumpPhysicalDevices(const std::vector<vk::PhysicalDevice> &devices) noexcept;
std::string physicalDeviceTypeToStr(vk::PhysicalDeviceType device_type) noexcept;
void selectPhysicalDeviceAndQueueFamily(const vk::Instance &instance,
                                        const std::vector<vk::PhysicalDevice> &devices,
                                        vk::PhysicalDevice *selected_device,
                                        uint32_t *queue_family_index, uint32_t *queue_count);
void dumpDeviceExtensions(const vk::PhysicalDevice &physical_device);

void mainLoop(GLFWwindow *window);
void cleanup(GLFWwindow *window);

extern "C" int run() {
    prepareGLFW();

    const auto instance_exts = getRequiredInstanceExtensions();
    const std::vector<const char *> layers = {"VK_LAYER_LUNARG_standard_validation"};
    const auto instance = createInstance(instance_exts, layers);

    const auto devices = instance->enumeratePhysicalDevices();
    if (devices.empty()) {
        std::cerr << "No physical device available for Vulkan" << std::endl;
        return EXIT_FAILURE;
    }
    dumpPhysicalDevices(devices);

    vk::PhysicalDevice physical_device;
    uint32_t queue_family_index;
    uint32_t queue_count;
    selectPhysicalDeviceAndQueueFamily(*instance, devices, &physical_device, &queue_family_index,
                                       &queue_count);
    std::cout << "Selected physical device: " << physical_device.getProperties().deviceName
              << std::endl
              << "Selected queue family (index): " << queue_family_index << std::endl;
    dumpDeviceExtensions(physical_device);

    const float queue_priorities[]{1.0f};
    const vk::DeviceQueueCreateInfo device_queue_create_infos[]{
        vk::DeviceQueueCreateInfo()
            .setQueueFamilyIndex(queue_family_index)
            .setQueueCount(queue_count)
            .setPQueuePriorities(queue_priorities)};

    // 論理デバイスを生成する
    const std::vector<const char *> device_exts = {VK_KHR_SWAPCHAIN_EXTENSION_NAME};
    const auto device =
        physical_device.createDevice(vk::DeviceCreateInfo()
                                         .setEnabledExtensionCount(device_exts.size())
                                         .setPpEnabledExtensionNames(device_exts.data())
                                         .setPpEnabledLayerNames(layers.data())
                                         .setQueueCreateInfoCount(1)
                                         .setPQueueCreateInfos(device_queue_create_infos));

    // コマンドプールを生成する
    // 描画命令等を保持するコマンドバッファが、コマンドプールから割り当てられる
    [[maybe_unused]] auto command_pool =
        device.createCommandPool(vk::CommandPoolCreateInfo()
                                     .setQueueFamilyIndex(queue_family_index)
                                     .setFlags(vk::CommandPoolCreateFlagBits::eResetCommandBuffer));

    // デバイスキューを取得する
    vk::Queue queue;
    device.getQueue(queue_family_index, 0, &queue);

    GLFWwindow *window = glfwCreateWindow(600, 500, "Application", nullptr, nullptr);

    VkSurfaceKHR surface;
    const auto result = glfwCreateWindowSurface(*instance, window, nullptr, &surface);
    if (result != VK_SUCCESS) {
        std::cerr << "Failed to create window surface" << std::endl;
        return EXIT_FAILURE;
    }

    // GPU のメモリの確保/解放
    // device.allocateMemoryUnique
    // device.allocateMemory
    // device.freeMemory

    // const auto cap = physical_device.getSurfaceCapabilitiesKHR(surface);

    // Vulkan にデフォルトフレームバッファの概念はないため、レンダリングしたいバッファを
    // 可視化する前に所有するインフラを必要とする。
    // このインフラはスワップチェインとして知られており、
    // Vulkan では明示的に生成されなければならない。
    // スワップチェインは、本質的には画面に表示されることを待機している画像のキューである。
    // アプリケーションはそのような画像を取得して描画し、キューに戻す。
    // キューがどのように機能するか、またキューから画像を提示する条件は、スワップチェーンがどのように設定されているかによって異なるが、
    // スワップチェーンの一般的な目的は、画像の提示を画面のリフレッシュレートに同期させることである。

    const auto surface_formats = physical_device.getSurfaceFormatsKHR(surface);
    std::cout << "Number of supported surface formats: " << surface_formats.size() << std::endl;

    mainLoop(window);
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

/** Vulkan インスタンスを生成する */
vk::UniqueHandle<vk::Instance, vk::DispatchLoaderStatic> createInstance(
    const std::vector<const char *> &instance_exts, const std::vector<const char *> &layers) {
    const auto app_info = vk::ApplicationInfo("Application", VK_MAKE_VERSION(0, 1, 0));
    const auto instance_create_info = vk::InstanceCreateInfo()
                                          .setPApplicationInfo(&app_info)
                                          .setEnabledExtensionCount(instance_exts.size())
                                          .setPpEnabledExtensionNames(instance_exts.data())
                                          .setPpEnabledLayerNames(layers.data());
    return vk::createInstanceUnique(instance_create_info);
}

/** 物理デバイスを一覧表示する */
void dumpPhysicalDevices(const std::vector<vk::PhysicalDevice> &devices) noexcept {
    std::cout << "Physical devices (" << devices.size() << "):" << std::endl;
    for (const auto &device : devices) {
        const auto props = device.getProperties();
        std::cout << "  " << props.deviceName << " (" << physicalDeviceTypeToStr(props.deviceType)
                  << ")" << std::endl;
    }
}

/** 物理デバイスの種別を表した文字列を取得する */
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

/** 使用する物理デバイスとキューファミリを決定する */
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

/** 物理デバイスが対応しているデバイス拡張を一覧表示する */
void dumpDeviceExtensions(const vk::PhysicalDevice &physical_device) {
    const auto device_extension_props = physical_device.enumerateDeviceExtensionProperties();
    std::cout << "Provided device extensions (" << device_extension_props.size()
              << "):" << std::endl;
    for (const auto &prop : device_extension_props)
        std::cout << "  " << prop.extensionName << std::endl;
}

/** ウィンドウを表示したあとの描画ループ */
void mainLoop(GLFWwindow *window) {
    while (!glfwWindowShouldClose(window)) {
        glfwPollEvents();
    }
}

/** 終了直前の処理 */
void cleanup(GLFWwindow *window) {
    glfwDestroyWindow(window);
    glfwTerminate();
}
