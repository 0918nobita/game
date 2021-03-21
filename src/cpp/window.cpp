// Copyright 2021 Kodai Matsumoto

#include "window.hpp"

#include <GLFW/glfw3.h>

void prepareGLFW() {
    glfwInit();
    glfwWindowHint(GLFW_CLIENT_API, GLFW_NO_API);
    glfwWindowHint(GLFW_RESIZABLE, 0);
}
