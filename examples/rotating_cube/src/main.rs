mod compose;
mod gameobjects;

extern crate gl;
extern crate nalgebra_glm as glm;

use compose::compose;
use microengine::{Backend::GLFW, GameConfig, Game};
use std::process;

const MAIN_SCENE: &str = "Rotating Cube";

pub fn game_config() -> GameConfig {
    GameConfig {
        backend: GLFW,
        window_cfg: Default::default(),
        fixed_fps: 50,
        starting_scene_name: MAIN_SCENE.into(),
    }
}

fn main() {
    let result = compose(Game::from(game_config())).run();
    match result {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}
