#pragma once

#include <GLFW/glfw3.h>

#include <functional>

class Window {
private:
    GLFWwindow *window;
public:
    Window(int width, int height, const char *title);
    /// 渡されたコールバック関数を毎回呼び出すイベントループを実行する
    void eventLoop(std::function<void(GLFWwindow*)> callback);
    ~Window();
};
