// Copyright 2021 Kodai Matsumoto

#include <type_traits>

constexpr int add_one(int n) { return n + 1; }

constexpr auto add_three(int n) {
    return [n] { return n + 3; };
}

template <int n>
struct Add1 {
    static constexpr int value = n + 1;
};

template <int n>
struct Mul2 : std::integral_constant<int, n * 2> {};

template <template <auto> typename G, template <auto> typename F>
struct Compose {
    template <auto Param>
    using value = G<F<Param>::value>;
};

template <template <auto> typename F>
using Twice = Compose<F, F>;

int main() {
    static_assert(add_one(2) == 3);
    static_assert(add_three(4)() == 7);
    static_assert(Add1<10>::value == 11);
    static_assert(Mul2<7>::value == 14);
    static_assert(Compose<Add1, Mul2>::value<3>::value == 7);
    static_assert(Compose<Mul2, Add1>::value<3>::value == 8);
    static_assert(Twice<Add1>::value<18>::value == 20);
    static_assert(Twice<Mul2>::value<10>::value == 40);
}
