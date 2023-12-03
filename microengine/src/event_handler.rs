use crate::{error::GameResult, input::Input, timer::Timer, window::Window};

pub mod glfw;

/// Enum representing implemented backends
/// Currently only GLFW Backend is implemented.
pub enum Backend {
    GLFW,
}

pub trait SystemEventFacade {
    /// Marks the start of game loop.
    /// Meant for polling events, updating window, input, and timer accordingly
    fn loop_start(
        &mut self,
        _window: &mut Window,
        _input: &mut Input,
        _timer: &mut Timer,
    ) -> GameResult {
        Ok(())
    }
    /// Marks the end of game loop.
    /// Meant for updating window, input, and timer accordingly
    fn loop_end(
        &mut self,
        _window: &mut Window,
        _input: &mut Input,
        _timer: &mut Timer,
    ) -> GameResult {
        Ok(())
    }
}
