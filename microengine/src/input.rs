pub mod keyboard;
pub mod mouse;

use keyboard::KeyBoard;
use mouse::Mouse;

/// Struct grouping input elements
pub struct Input {
    pub kb: KeyBoard,
    pub mouse: Mouse,
}

impl Input {
    pub fn new() -> Self {
        Input {
            kb: KeyBoard::default(),
            mouse: Mouse::default(),
        }
    }

    /// Updates all input elements after finished frame
    pub(crate) fn update_state(&mut self) {
        self.kb.update_key_state();
        self.mouse.update_key_state();
    }
}

impl Default for Input {
    fn default() -> Self {
        Input::new()
    }
}

/// Enum used for keys/buttons state polling
#[derive(Clone, Copy)]
pub enum KeyState {
    Down,
    Pressed,
    Up,
    NotPressed,
}

impl From<KeyState> for bool {
    fn from(value: KeyState) -> Self {
        match value {
            KeyState::Down | KeyState::Pressed => true,
            _ => false,
        }
    }
}

// TODO! Add configurable axis like in unity
