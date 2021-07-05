extern crate game;

use game::Application;

fn main() {
    env_logger::init();
    Application::new().unwrap().run();
}
