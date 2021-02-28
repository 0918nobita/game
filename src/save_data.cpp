// Copyright 2021 Kodai Matsumoto

#include "save_data.hpp"

#include <fstream>
#include <iostream>
#include <nlohmann/json.hpp>

using json = nlohmann::json;

void write_save_data() {
    json scene1 = {{"part", 1}, {"chapter", 1}, {"section", 1}};
    json scene2 = {{"part", 1}, {"chapter", 1}, {"section", 2}};
    json dat = {{"recent_scene", scene2}};
    dat["read_scene"] = {scene1, scene2};
    std::cout << dat << std::endl;

    const auto msgpack = json::to_msgpack(dat);
    std::ofstream out("save.data");
    out.write((const char*)msgpack.data(), msgpack.size());
}
