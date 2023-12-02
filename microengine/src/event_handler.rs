use crate::{window::Window, input::Input, error::GameResult, timer::Timer};

pub mod glfw;

/// Enum representing implemented backends 
/// Currently only GLFW Backend is implemented.
pub enum Backend {
    GLFW,
}

pub trait SystemEventFacade {
    /// Marks the start of game loop.
    /// Meant for polling events, updating window, input, and timer accordingly
    fn loop_start(&mut self, window: &mut Window, input: &mut Input, timer: &mut Timer) -> GameResult;
    /// Marks the end of game loop.
    /// Meant for updating window, input, and timer accordingly
    fn loop_end(&mut self, window: &mut Window, input: &mut Input, timer: &mut Timer) -> GameResult;
}
