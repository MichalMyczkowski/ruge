use clap::Parser;
use microengine::{Backend::GLFW, GameConfig};

pub const MAX_HEIGHT: i32 = 11_000;
pub const MIN_HEIGHT: i32 = -11_000;

pub const MAIN_SCENE: &str = "earth";
static mut DEBUG: bool = false;

pub fn debug() -> bool {
    unsafe {
        DEBUG
    }
}

/// Sixth task during Computer Graphics course.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct EarthConfig {
    /// Run game in debug mode
    #[arg(short, long, default_value_t = false)]
    debug: bool
}

pub fn earth_config() -> EarthConfig {
    let c = EarthConfig::parse();
    unsafe {
        DEBUG = c.debug;
        gl::Enable(gl::CULL_FACE);
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
