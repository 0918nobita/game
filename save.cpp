// Copyright 2021 Kodai Matsumoto

#include <google/protobuf/repeated_field.h>
#include <fstream>
#include <iostream>
#include "save_data.pb.h"

int main(int argc, char* argv[]) {
    save_data::SaveData_Scene scene1;
    scene1.set_part(1);
    scene1.set_chapter(1);
    scene1.set_section(1);

    save_data::SaveData_Scene scene2;
    scene2.set_part(1);
    scene2.set_chapter(1);
    scene2.set_section(2);

    google::protobuf::RepeatedPtrField<save_data::SaveData_Scene> scenes;
    auto first = scenes.Add();
    first->CopyFrom(scene1);
    auto second = scenes.Add();
    second->CopyFrom(scene2);

    save_data::SaveData dat;
    auto read_scenes = dat.mutable_read_scenes();
    read_scenes->CopyFrom(scenes);
    auto recent_scene = dat.mutable_recent_scene();
    recent_scene->CopyFrom(scene2);

    std::ofstream ofs("save.data");
    dat.SerializeToOstream(&ofs);

    return EXIT_SUCCESS;
}
