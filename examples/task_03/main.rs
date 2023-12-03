mod config;
mod compose;
mod gameobjects;

use std::process;
use microengine::Game;
use compose::compose;

pub const MAIN_SCENE: &str = "labirynth"; 

fn main() {
    let lc = config::labirynth_config();
    let result = compose(Game::from(config::game_config()), lc).run();
    match result {
        Ok(_) => (),
        Err(e) => { 
            eprintln!("{}", e);
            process::exit(1);
        },
    }
}
