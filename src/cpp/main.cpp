// Copyright 2021 Kodai Matsumoto

#define GLFW_INCLUDE_VULKAN
#include <GLFW/glfw3.h>

#include <algorithm>
#include <iostream>
#include <vulkan/vulkan.hpp>

#include "physical_device.hpp"
#include "vk_instance.hpp"
#include "window.hpp"

#ifdef _MSC_VER
#define DLL_EXPORT __declspec(dllexport)
#else
#define DLL_EXPORT
#endif

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

extern "C" int DLL_EXPORT run() {
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
    const auto device = physical_device.createDevice(
        vk::DeviceCreateInfo()
            .setEnabledExtensionCount(static_cast<uint32_t>(device_exts.size()))
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
