use super::KeyState;
use std::{cell::RefCell, iter};

#[derive(Clone, Copy)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

impl From<MouseButton> for usize {
    fn from(value: MouseButton) -> Self {
        match value {
            MouseButton::Left => 0,
            MouseButton::Right => 1,
            MouseButton::Middle => 2,
        }
    }
}

pub struct Mouse {
    /// current mouse position (x, y) in pixel coordinates
    pub position: (f32, f32),
    /// difference in mouse position (x, y) from last frame
    pub position_delta: (f32, f32),
    /// difference in mouse scroll from last frame.
    pub scroll_delta: (f32, f32),

    pub(crate) cursor_visibility: RefCell<bool>,
    buttons: Vec<KeyState>,
}

impl Mouse {
    pub(crate) fn new(cursor_visible: bool) -> Self {
        Self {
            position: (0.0, 0.0),
            position_delta: (0.0, 0.0),
            scroll_delta: (0.0, 0.0),
            cursor_visibility: RefCell::new(cursor_visible),
            buttons: iter::repeat_with(|| KeyState::NotPressed).take(3).collect(),
        }
    }

    /// Change button state to KeyState::Down if not pressed already
    /// Managed by SystemEventFacade implementation
    pub(crate) fn press_button(&mut self, button: MouseButton) {
        self.buttons[button as usize] = match self.buttons[button as usize] {
            KeyState::Up | KeyState::NotPressed => KeyState::Down,
            _ => KeyState::Pressed,
        };
    }

    /// Change button state to KeyState::Up
    /// Managed by SystemEventFacade implementation
    pub(crate) fn release_button(&mut self, button: MouseButton) {
        self.buttons[button as usize] = KeyState::Up;
    }

    /// updates all buttons released in last frame to KeyState::NotPressed
    /// and all buttons pressed in last frame to KeyState::Pressed
    pub(crate) fn update_key_state(&mut self) {
        self.buttons.iter_mut().for_each(|k| {
            *k = match *k {
                KeyState::Up => KeyState::NotPressed,
                KeyState::Down => KeyState::Pressed,
                x => x,
            };
        });
    }

    // public API

    // TODO? Enum for visible, hidden, disabled
    /// Set or unset cursor visibility
    pub fn set_cursor_visibility(&self, visible: bool) {
        let mut v = self.cursor_visibility.borrow_mut();
        *v = visible;
    }

    /// Check if cursor is visible
    pub fn is_cursor_visible(&self) -> bool {
        *self.cursor_visibility.borrow()
    }

    /// Returns true during the frame the user starts pressing down the button.
    pub fn get_key_down(&self, button: MouseButton) -> bool {
        match self.buttons[button as usize] {
            KeyState::Down => true,
            _ => false,
        }
    }

    /// Returns true during the frame the user releases the button.
    pub fn get_key_up(&self, button: MouseButton) -> bool {
        match self.buttons[button as usize] {
            KeyState::Up => true,
            _ => false,
        }
    }

    /// Returns true while the user holds down the key identified by KeyCode.
    pub fn get_key(&self, button: MouseButton) -> bool {
        match self.buttons[button as usize] {
            KeyState::Pressed => true,
            _ => false,
        }
    }
}

impl Default for Mouse {
    fn default() -> Self {
        Mouse::new(true)
    }
}
