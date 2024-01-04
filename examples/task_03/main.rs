mod compose;
mod config;
mod drawable;
mod gameobjects;
mod utils;

extern crate gl;
extern crate nalgebra_glm as glm;
extern crate rand;

use compose::compose;
use microengine::Game;
use std::process;

pub const MAIN_SCENE: &str = "labirynth";

fn main() {
    let lc = config::labirynth_config();
    let result = compose(Game::from(config::game_config()), lc).run();
    match result {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}
