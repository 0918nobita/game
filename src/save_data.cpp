// Copyright 2021 Kodai Matsumoto

#include "save_data.hpp"

#include <fstream>
#include <iostream>
#include <iterator>
#include <nlohmann/json.hpp>

using json = nlohmann::json;

bool operator==(const Scene& lhs, const Scene& rhs) {
    return lhs.part == rhs.part && lhs.chapter == rhs.chapter && lhs.section == rhs.section;
}

void to_json(json& j, const Scene& scene) {
    j = {{"part", scene.part}, {"chapter", scene.chapter}, {"section", scene.section}};
}

void from_json(const json& j, Scene& scene) {
    j.at("part").get_to(scene.part);
    j.at("chapter").get_to(scene.chapter);
    j.at("section").get_to(scene.section);
}

bool operator==(const SaveData& lhs, const SaveData& rhs) {
    return lhs.read_scenes == rhs.read_scenes && lhs.recent_scene == rhs.recent_scene;
}

void to_json(json& j, const SaveData& save_data) {
    j = {{"read_scenes", save_data.read_scenes}, {"recent_scene", save_data.recent_scene}};
}

void from_json(const json& j, SaveData& save_data) {
    j.at("read_scenes").get_to(save_data.read_scenes);
    j.at("recent_scene").get_to(save_data.recent_scene);
}

void write_save_data() {
    const SaveData save_data = {.read_scenes = {{1, 1, 1}, {1, 1, 2}}, .recent_scene = {1, 1, 2}};
    const json dat = save_data;
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

    const SaveData save_data = dat;
    const SaveData expected = {.read_scenes = {{1, 1, 1}, {1, 1, 2}}, .recent_scene = {1, 1, 2}};
    assert(save_data == expected);
}
