use microengine::{Backend::GLFW, GameConfig};

pub const MAIN_SCENE: &str = "3d_maze";

pub fn game_config() -> GameConfig {
    GameConfig {
        backend: GLFW,
        window_cfg: Default::default(),
        fixed_fps: 50,
        starting_scene_name: MAIN_SCENE.into(),
    }
}
