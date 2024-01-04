mod compose;
mod config;
mod gameobjects;

extern crate gl;
extern crate nalgebra_glm as glm;
extern crate rand;

use compose::compose;
use microengine::Game;
use std::process;


fn main() {
    let result = compose(Game::from(config::game_config())).run();
    match result {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}
