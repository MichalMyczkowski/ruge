//! This module is responsible for glueing everything else provided into an actual game
//! methods implemented on Game are responsible for compositing (holding and switching between)
//! scenes, polling provided backend and what's most important: running your game!
//!
use std::collections::HashMap;

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
    scenes: HashMap<String, Scene>,
    ev_handler: Box<dyn SystemEventFacade>,
    next_scene_name: Option<String>,
}

impl Game {
    /// Pretty self explanatory
    pub fn run(&mut self) -> GameResult {
        let mut active_scene: Option<Scene> = None;
        while !self.ctx.window.should_close() {
            self.ev_handler.loop_start(
                &mut self.ctx.window,
                &mut self.ctx.input,
                &mut self.ctx.time,
            )?;
            if let Some(ref name) = self.next_scene_name.take() {
                if !self.scenes.contains_key(name) {
                    return Err(GameError::GameLogicError(format!("There is no scene named: {name}")));
                }
                let prev_scene = active_scene.replace(
                    self.scenes.remove(name).unwrap()
                );
                if let Some(s) = prev_scene {
                    if !s.disposable {
                        let s = self.scenes.insert(s.name.clone(), s);
                        if let Some(s) = s {
                            return Err(
                                GameError::GameLogicError(
                                    format!(
                                        "Can't add more than one scene with name: {}",
                                        s.name
                                    )
                                )
                            )
                        }
                    }
                }
            }
            
            // RUN SCENE
            // UPDATE SCENES (add dynamically created scenes)
            match active_scene {
                Some(ref mut scene) => {
                    scene.run_loop(&mut self.ctx)?;
                    // TODO!
                    // GET ALL DYNAMICALLY CREATED SCENES FROM SCENE!
                    // CHECK IF SCENE SHOULD CHANGE ( scene.should_change()-> Option<String> )
                },
                None => {
                    return Err(
                        GameError::GameLogicError("Trying to run game without setting starting scene first!".into())
                    );
                }
            }

            self.ev_handler.loop_end(
                &mut self.ctx.window,
                &mut self.ctx.input,
                &mut self.ctx.time,
            )?;
        }
        Ok(())
    }
    
    /// Adds given scene to the game.
    /// Use this method to compose your game!
    pub fn add_scene(&mut self, scene: Scene) -> GameResult {
        let s = self.scenes.insert(scene.name.clone(), scene);
        match s {
            Some(s) => {
                Err(
                    GameError::GameLogicError(
                        format!(
                            "Can't add more than one scene with name: {}",
                            s.name
                        )
                    )
                )
            },
            None => Ok(()),
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
            scenes: HashMap::new(),
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
            scenes: HashMap::new(),
            ev_handler: backend,
            next_scene_name: None,
        }
    }

    struct DoNothingBackend;
    impl SystemEventFacade for DoNothingBackend {}

    struct RequestGameClose;
    impl GameObject for RequestGameClose {
        fn update(&mut self, ctx: &Context, _scene: &Scene) -> GameResult {
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
