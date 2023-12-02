//! Provides GameObject trait which handles gameloop events
use std::any::Any;
use crate::error::GameResult;
use crate::context::Context;

#[derive(Hash, Clone, PartialEq, Eq)]
pub struct GameObjectId {
    pub(crate) layer: usize,
    pub(crate) id: usize,
}


pub trait GameObject {
    /// start is executed only on scene start
    /// it will not be executed on any gameobject created after scene starts
    fn start(&mut self, _ctx: &Context) -> GameResult { Ok(()) }
    
    /// function called every frame
    fn update(&mut self, _ctx: &Context) -> GameResult { Ok(()) }
    
    /// function simulating fixed time step
    /// default time step is 1/50s but it is configurable with GameConf
    fn fixed_update(&mut self, _ctx: &Context) -> GameResult { Ok(()) }

    /// if is_dead returns true Scene will clean this gameobject
    fn is_dead(&mut self) -> bool { false }

    /// update but is called on scene change or game end
    /// scene will change only after all gameobjects return true from finished_update
    /// Use it for elegant shut down (animations and such)
    fn finished_update(&mut self, _ctx: &Context) -> GameResult<bool> { Ok(true) }

    /// last method called in every gameloop
    /// use it to draw to screen
    fn draw(&mut self, _ctx: &Context) -> GameResult { Ok(()) }

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
