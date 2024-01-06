use clap::Parser;
use microengine::{Backend::GLFW, GameConfig};

pub const MAIN_SCENE: &str = "3d_maze";
static mut DEBUG: bool = false;

pub fn debug() -> bool {
    unsafe {
        DEBUG
    }
}

/// Fourth task during Computer Graphics course.
/// A 3D game, where your goal is to reach the tetrahedron located at the end of maze.
/// CONTROLS:
/// W,S,A,D -> player movement,
/// Mouse -> camera movement,
/// TAB -> switch between different cameras
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct MazeConfig {
    /// Seed for generating random maze
    #[arg(long, default_value_t = String::from("aa"))]
    pub seed: String,
    /// Maze size (size x size x size tetrahedrons)
    #[arg(long, default_value_t = 5)]
    pub size: usize,
    /// Run game in debug mode
    #[arg(short, long, default_value_t = false)]
    debug: bool
}

pub fn maze_config() -> MazeConfig {
    let c = MazeConfig::parse();
    unsafe {
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
