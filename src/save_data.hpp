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
};

bool operator==(const Scene& lhs, const Scene& rhs);

void to_json(json& j, const Scene& scene);
void from_json(const json& j, Scene& scene);

struct SaveData {
    std::vector<Scene> read_scenes;
    Scene recent_scene;
};

bool operator==(const SaveData& lhs, const SaveData& rhs);

void to_json(json& j, const SaveData& save_data);
void from_json(const json& j, SaveData& save_data);

void write_save_data();
void read_save_data();
