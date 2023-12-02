pub mod keyboard;

use keyboard::KeyBoard;

/// Struct grouping input elements
pub struct Input {
    pub kb: KeyBoard,
}

impl Input {
    pub fn new() -> Self {
        Input {
            kb: KeyBoard::default(),
        }
    }
}

impl Default for Input {
    fn default() -> Self {
        Input::new()
    }
}
