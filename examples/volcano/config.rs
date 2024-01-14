use clap::Parser;
use microengine::{Backend::GLFW, GameConfig};

pub const MAIN_SCENE: &str = "volcano";
static mut DEBUG: bool = false;

pub fn debug() -> bool {
    unsafe {
        DEBUG
    }
}

/// Fifth task during Computer Graphics course.
/// Collect 16 green glowing bubbles and reach the center of the volcano.
/// CONTROLS:
/// W,S,A,D -> player movement,
/// Mouse -> camera movement,
/// TAB -> switch between different cameras
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct VolcanoConfig {
    /// Seed for generating random bubble position
    #[arg(long, default_value_t = String::from("aa"))]
    pub seed: String,
    /// Run game in debug mode
    #[arg(short, long, default_value_t = false)]
    debug: bool
}

pub fn volcano_config() -> VolcanoConfig {
    let c = VolcanoConfig::parse();
    unsafe {
        //gl::Enable(gl::CULL_FACE); 
        DEBUG = c.debug;
    }
    c
}

pub fn game_config() -> GameConfig {
    GameConfig {
        backend: GLFW,
        window_cfg: Default::default(),
        fixed_fps: 50,
        starting_scene_name: MAIN_SCENE.into(),
    }
}
