import mymodule;

#define UNUSED __attribute__((unused))

// #define GLFW_INCLUDE_VULKAN
#include <GLFW/glfw3.h>

#include <iostream>
#include <memory>

#include "window.hpp"

int main() {
    std::cout << add3(4) << std::endl;
    std::unique_ptr<Window> window(new Window(600, 500, "Game"));
    (*window).eventLoop([](UNUSED GLFWwindow *window) {});
    return EXIT_SUCCESS;
}