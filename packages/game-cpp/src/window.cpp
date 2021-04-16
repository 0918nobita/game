#include <GLFW/glfw3.h>

#include <functional>
#include <memory>

#include "window.hpp"

GLFWwindow *Window::initWindow(int width, int height, const char *title) {
    glfwInit();
    glfwWindowHint(GLFW_CLIENT_API, GLFW_NO_API);
    glfwWindowHint(GLFW_RESIZABLE, 0);
    return glfwCreateWindow(width, height, title, nullptr, nullptr);
}

Window::Window(int width, int height, const char *title) :
    window(std::make_shared<GLFWwindow *>(initWindow(width, height, title))) {}

void Window::eventLoop(std::function<void (std::shared_ptr<GLFWwindow *>)> callback) {
    while (!glfwWindowShouldClose(*window)) {
        glfwPollEvents();
        callback(window);
    }
}

Window::~Window() {
    glfwDestroyWindow(*window);
    glfwTerminate();
}
