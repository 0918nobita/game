#define UNUSED __attribute__((unused))

#define GLFW_INCLUDE_VULKAN
#include <GLFW/glfw3.h>

#include <iostream>
#include <iterator>
#include <memory>
#include <vector>

#include "window.hpp"

std::vector<const char *> getRequiredInstanceExtensions() noexcept {
    uint32_t num_required_exts;
    auto required_exts = glfwGetRequiredInstanceExtensions(&num_required_exts);
    std::vector<const char *> extensions(num_required_exts);
    for (uint32_t i = 0; i < num_required_exts; i++) extensions[i] = required_exts[i];
    return extensions;
}

int main() {
    const auto instance_exts = getRequiredInstanceExtensions();
    Window window(600, 500, "Game");
    window.eventLoop([](UNUSED GLFWwindowPtr window) {});
    return EXIT_SUCCESS;
}
