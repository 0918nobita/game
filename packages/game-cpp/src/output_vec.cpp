#include <iostream>
#include <vector>

template <class T>
concept OstreamOutputable = requires(std::ostream& os, T a) {
    os << a;
};

template <OstreamOutputable T>
std::ostream& operator<<(std::ostream& os, const std::vector<T> vec) {
    for (const auto &item : vec) os << item << std::endl;
    return os;
}

int main() {
    std::cout << std::vector<int>({ 10, 20, 30 });
    return EXIT_SUCCESS;
}
