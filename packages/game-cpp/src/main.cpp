#define UNUSED __attribute__((unused))

#include <memory>

#include "window.hpp"

int main() {
    Window window(600, 500, "Game");
    window.eventLoop([](UNUSED std::shared_ptr<GLFWwindow *> window) {});
    return EXIT_SUCCESS;
}
