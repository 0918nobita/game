#pragma once

#include <GLFW/glfw3.h>

#include <functional>
#include <memory>

class Window {
private:
    std::shared_ptr<GLFWwindow *> window;
    static GLFWwindow *initWindow(int width, int height, const char* title);
public:
    Window(int width, int height, const char *title);
    /// 渡されたコールバック関数を毎回呼び出すイベントループを実行する
    void eventLoop(std::function<void (std::shared_ptr<GLFWwindow *>)> callback);
    ~Window();
};
