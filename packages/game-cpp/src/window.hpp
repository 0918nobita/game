#pragma once

#include <GLFW/glfw3.h>

#include <functional>
#include <memory>

using GLFWwindowPtr = std::shared_ptr<GLFWwindow *>;

class Window {
private:
    GLFWwindowPtr window;
    static GLFWwindow *initWindow(int width, int height, const char* title);
public:
    Window(int width, int height, const char *title);
    /// 渡されたコールバック関数を毎回呼び出すイベントループを実行する
    void eventLoop(std::function<void (GLFWwindowPtr)> callback);
    ~Window();
};
