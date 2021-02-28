// Copyright 2021 Kodai Matsumoto

#include <fstream>
#include <iostream>

#include "save_data.pb.h"

int main() {
    std::ifstream fin("save.data");
    if (!fin) {
        std::cerr << "Failed to open ./save.data" << std::endl;
        return EXIT_FAILURE;
    }
    save_data::SaveData dat;
    if (!dat.ParseFromIstream(&fin)) {
        std::cerr << "Failed to write ./save.data" << std::endl;
        return EXIT_FAILURE;
    }
    std::cout << dat.DebugString();
    return EXIT_SUCCESS;
}
