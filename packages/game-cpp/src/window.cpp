#include <GLFW/glfw3.h>

#include <functional>

#include "window.hpp"

Window::Window(int width, int height, const char *title) {
    glfwInit();
    glfwWindowHint(GLFW_CLIENT_API, GLFW_NO_API);
    glfwWindowHint(GLFW_RESIZABLE, 0);
    window = glfwCreateWindow(width, height, title, nullptr, nullptr);
}

void Window::eventLoop(std::function<void(GLFWwindow*)> callback) {
    while (!glfwWindowShouldClose(window)) {
        glfwPollEvents();
        callback(window);
    }
}

Window::~Window() {
    glfwDestroyWindow(window);
    glfwTerminate();
}
