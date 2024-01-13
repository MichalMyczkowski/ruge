//! Provides GameObject trait which handles gameloop events
use crate::context::Context;
use crate::error::GameResult;
use crate::Scene;
use std::any::Any;

#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug)]
pub struct GameObjectId {
    pub(crate) layer: usize,
    pub(crate) idx: usize,
    pub(crate) id: usize,
}

pub trait GameObject {

    /// on_add is executed when gameobejct is added to scene
    /// its id is given as one of the arguments, so it can be used later.
    fn on_add(&mut self, _ctx: &Context, _scene: &Scene, _id: GameObjectId) -> GameResult {
        Ok(())
    }

    /// start is executed only once when the scene is first started.
    /// Only gameobjects added before running the scene will execute this method.
    fn start(&mut self, _ctx: &Context, _scene: &Scene) -> GameResult {
        Ok(())
    }

    /// function called every frame
    fn update(&mut self, _ctx: &Context, _scene: &Scene) -> GameResult {
        Ok(())
    }

    /// function simulating fixed time step
    /// default time step is 1/50s but it is configurable with GameConf
    fn fixed_update(&mut self, _ctx: &Context, _scene: &Scene) -> GameResult {
        Ok(())
    }

    /// if is_dead returns true Scene will clean this gameobject
    fn is_dead(&mut self) -> bool {
        false
    }

    /// last method called in every gameloop
    /// use it to draw to screen
    fn draw(&mut self, _ctx: &Context, _scene: &Scene) -> GameResult {
        Ok(())
    }

    fn name(&self) -> &str {
        ""
    }

    /// as_any is needed to make searching for other gameobjects possible
    /// with scene.gameobject_by_id<T>(id).unwrap();
    /// implement as:
    /// ```rust=
    /// fn as_any(&self) -> &dyn Any {
    ///     self
    /// }
    /// ```
    fn as_any(&self) -> &dyn Any;
}
