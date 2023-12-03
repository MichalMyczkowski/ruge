
use clap::Parser;
use crate::MAIN_SCENE;
use microengine::{GameConfig, Backend::GLFW};

/// Third task during Computer Graphics course
/// A simple labirynth game, where your goal is to reach
/// the triangle in the top right corner.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct LabirynthConfig {
    /// Seed for generating random labirynth
    #[arg(long)]
    pub seed: String,
    /// Labirynth size (size x size triangles)
    #[arg(long, default_value_t = 15)] 
    pub size: usize,
}

pub fn labirynth_config() -> LabirynthConfig {
    LabirynthConfig::parse()
}

pub fn game_config() -> GameConfig {
    GameConfig {
        backend: GLFW,
        window_cfg: Default::default(), 
        fixed_fps: 50,
        starting_scene_name: MAIN_SCENE.into(),
    }
}
