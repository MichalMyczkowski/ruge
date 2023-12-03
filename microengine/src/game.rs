//! This module is responsible for glueing everything else provided into an actual game
//! methods implemented on Game are responsible for compositing (holding and switching between)
//! scenes, polling provided backend and what's most important: running your game!
//!
use crate::{
    context::Context,
    error::{GameError, GameResult},
    event_handler::{glfw::GLFWBackend, Backend, SystemEventFacade},
    scene::Scene,
    timer::Timer,
    window::{Window, WindowConfig},
};

pub struct GameConfig {
    pub backend: Backend,
    pub window_cfg: WindowConfig,
    pub fixed_fps: usize,
    pub starting_scene_name: String,
}

pub struct Game {
    ctx: Context,
    // TODO! change from Vec<Scene> to HashMap<String, Scene>
    scenes: Vec<Scene>,
    ev_handler: Box<dyn SystemEventFacade>,
    next_scene_name: Option<String>,
}

impl Game {
    /// Pretty self explanatory
    pub fn run(&mut self) -> GameResult {
        while !self.ctx.window.should_close() {
            self.ev_handler.loop_start(
                &mut self.ctx.window,
                &mut self.ctx.input,
                &mut self.ctx.time,
            )?;
            if self.next_scene_name.is_some() {
                let name = self.next_scene_name.take().unwrap();
                // TODO! add scenes from ctx (created at runtime) to self.scenes first
                let idx: Vec<usize> = self
                    .scenes
                    .iter()
                    .enumerate()
                    .filter(|(_, s)| s.name == name)
                    .map(|(idx, _)| idx)
                    .collect();
                if idx.len() == 0 {
                    return Err(GameError::GameLogicError(format!(
                        "Trying to transition to non existent scene: {}",
                        name
                    )));
                } else if idx.len() > 1 {
                    return Err(GameError::GameLogicError(format!(
                        "There are more than one scene with name: {}",
                        name
                    )));
                }
                let idx = idx[0];
                let scene = self.scenes.remove(idx);
                if let Some(scene) = self.ctx.set_scene(scene)? {
                    if !scene.disposable {
                        self.scenes.push(scene);
                    }
                }
            }

            if self.ctx.run_current_scene()? {
                self.next_scene_name = self.ctx.scene_should_change();
            }

            self.ev_handler.loop_end(
                &mut self.ctx.window,
                &mut self.ctx.input,
                &mut self.ctx.time,
            )?;
        }
        Ok(())
    }

    /// Use this method to compose your game!
    pub fn add_scene(&mut self, scene: Scene) -> GameResult {
        if self.scenes.iter().filter(|s| s.name == scene.name).count() != 0 {
            Err(GameError::GameLogicError(format!(
                "Can't add more than one scene with name: {}",
                scene.name
            )))
        } else {
            self.scenes.push(scene);
            Ok(())
        }
    }

    pub fn set_starting_scene_name(&mut self, scene_name: &str) {
        self.next_scene_name = Some(scene_name.into());
    }
}

/// For now it's the only way to create a Game
impl From<GameConfig> for Game {
    fn from(value: GameConfig) -> Self {
        let window = Window::from(value.window_cfg);
        let backend = match value.backend {
            Backend::GLFW => GLFWBackend::new(&window),
        };
        Game {
            ctx: Context::new(Timer::new(value.fixed_fps), window),
            scenes: Vec::new(),
            ev_handler: Box::new(backend),
            next_scene_name: Some(value.starting_scene_name),
        }
    }
}

// TODO! implement mock backend and test Game struct
#[cfg(test)]
mod tests {
    use crate::{input::Input, GameObject};

    use super::*;
    fn game_from_backend(backend: Box<dyn SystemEventFacade>) -> Game {
        Game {
            ctx: Default::default(),
            scenes: Vec::new(),
            ev_handler: backend,
            next_scene_name: None,
        }
    }

    struct DoNothingBackend;
    impl SystemEventFacade for DoNothingBackend {}

    struct RequestGameClose;
    impl GameObject for RequestGameClose {
        fn update(&mut self, ctx: &Context) -> GameResult {
            ctx.window.close();
            Ok(())
        }
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    #[test]
    fn window_close_shutsdown_game() {
        let backend = DoNothingBackend;
        let mut g = game_from_backend(Box::new(backend));
        let mut s = Scene::default();
        _ = s.add_gameobject(RequestGameClose, 0);
        g.set_starting_scene_name(&s.name);
        _ = g.add_scene(s);
        assert_eq!(true, g.run().is_ok())
    }

    struct SystemCloseBackend;
    impl SystemEventFacade for SystemCloseBackend {
        fn loop_start(
            &mut self,
            window: &mut Window,
            _input: &mut Input,
            _timer: &mut Timer,
        ) -> GameResult {
            window.system_close();
            Ok(())
        }
    }

    struct DoNothingGameObject;
    impl GameObject for DoNothingGameObject {
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    #[test]
    fn window_system_close_shutsdown_game() {
        let backend = SystemCloseBackend;
        let mut g = game_from_backend(Box::new(backend));
        let mut s = Scene::default();
        _ = s.add_gameobject(DoNothingGameObject, 0);
        g.set_starting_scene_name(&s.name);
        _ = g.add_scene(s);
        assert_eq!(true, g.run().is_ok())
    }
}
