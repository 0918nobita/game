#define UNUSED __attribute__((unused))

#include <memory>

#include "window.hpp"

int main() {
    Window window(600, 500, "Game");
    window.eventLoop([](UNUSED GLFWwindowPtr window) {});
    return EXIT_SUCCESS;
}
