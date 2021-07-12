extern crate game;

#[macro_use]
extern crate anyhow;

use ash::Entry;
use game::{glfw_wrapper::GlfwWrapper, instance::ManagedInstance};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(PartialEq, Serialize, Deserialize)]
struct SaveData {
    name: String,
    high_score: u32,
}

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let save_data = SaveData {
        name: "kodai".to_owned(),
        high_score: 200,
    };
    let serialized = bincode::serialize(&save_data)?;
    fs::write("save_data", &serialized)?;

    let content = fs::read("save_data")?;
    let deserialized: SaveData = bincode::deserialize(&content)?;
    ensure!(save_data == deserialized);

    let entry = unsafe { Entry::new() }?;
    let glfw = GlfwWrapper::new()?;
    let instance = ManagedInstance::new(&entry, &glfw, cfg!(feature = "validation_layers"))?;
    let _window = instance.create_window(500, 300, "Game")?;
    let _physical_device = instance.find_physical_device(|_device| true)?;
    Ok(())
}
