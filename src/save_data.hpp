// Copyright 2021 Kodai Matsumoto

#pragma once

#include <iostream>
#include <nlohmann/json.hpp>
#include <vector>

using json = nlohmann::json;

struct Scene {
    int part;
    int chapter;
    int section;
    bool operator==(const Scene&) const = default;
};

void to_json(json&, Scene&);
void from_json(const json&, Scene&);

struct SaveData {
    std::vector<Scene> read_scenes;
    Scene recent_scene;
    bool operator==(const SaveData&) const = default;
};

void to_json(json&, const SaveData&);
void from_json(const json&, SaveData&);

void write_save_data();
void read_save_data();
