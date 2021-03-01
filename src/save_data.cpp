// Copyright 2021 Kodai Matsumoto

#include "save_data.hpp"

#include <fstream>
#include <iostream>
#include <iterator>
#include <nlohmann/json.hpp>

using json = nlohmann::json;

void write_save_data() {
    json scene1 = {{"part", 1}, {"chapter", 1}, {"section", 1}};
    json scene2 = {{"part", 1}, {"chapter", 1}, {"section", 2}};
    json dat = {{"recent_scene", scene2}};
    dat["read_scenes"] = {scene1, scene2};
    std::cout << "write:\n" << dat.dump(2) << std::endl;

    const auto msgpack = json::to_msgpack(dat);
    std::ofstream fout("save.data");
    fout.write((const char*)msgpack.data(), msgpack.size());
}

void read_save_data() {
    std::ifstream fin("save.data");
    if (!fin) {
        std::cerr << "Failed to open ./save.data" << std::endl;
        std::exit(EXIT_FAILURE);
    }
    std::vector<uint8_t> msgpack;
    std::copy(std::istreambuf_iterator<char>(fin), std::istreambuf_iterator<char>(),
              std::back_inserter(msgpack));
    const auto dat = json::from_msgpack(msgpack);
    std::cout << "\nread:\n" << dat.dump(2) << std::endl;
}
