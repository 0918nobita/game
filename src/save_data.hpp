// Copyright 2021 Kodai Matsumoto

#pragma once

#include <vector>

struct Scene {
    int part;
    int chapter;
    int section;
};

struct SaveData {
    std::vector<Scene> read_scenes;
    Scene recent_scene;
};

void write_save_data();
void read_save_data();
