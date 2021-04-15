#define UNUSED __attribute__((unused))

// #define GLFW_INCLUDE_VULKAN
#include <GLFW/glfw3.h>

#include <cassert>
#include <iostream>
#include <memory>
#include <string>

#include "window.hpp"

// std::vector<const char *> getRequiredInstanceExtensions() noexcept;

int main() {
    std::string str; // 空文字列を生成する。
    assert(str == "");

    std::unique_ptr<Window> window(new Window(600, 500, "Game"));
    (*window).eventLoop([](UNUSED GLFWwindow *window) {});
    return EXIT_SUCCESS;
}
