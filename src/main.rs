extern crate game;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let _glfw = &game::GLFW;
    Ok(())
}
