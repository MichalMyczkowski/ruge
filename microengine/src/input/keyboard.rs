//! Module providing Keyboard struct which is used to get user input through methods like:
//! get_key_down, get_key and get_key_up
pub mod keys;
use std::iter;

use keys::{KeyCode, KeyState, NUM_KEYS};

pub struct KeyBoard {
    keys: Vec<KeyState>,
}

/// KeyBoard events should be updated here by SystemEventFacade implementations
impl KeyBoard {
    pub(crate) fn new() -> Self {
        KeyBoard {
            keys: iter::repeat_with(|| KeyState::NotPressed)
                .take(NUM_KEYS)
                .collect(),
        }
    }

    /// Change key state to KeyState::Down
    /// Managed by SystemEventFacade implementation
    pub(crate) fn press_key(&mut self, key: KeyCode) {
        self.keys[key as usize] = match self.keys[key as usize] {
            KeyState::Up | KeyState::NotPressed => KeyState::Down,
            _ => KeyState::Pressed,
        };
    }

    /// Change key state to KeyState::Up
    /// Managed by SystemEventFacade implementation
    pub(crate) fn release_key(&mut self, key: KeyCode) {
        self.keys[key as usize] = KeyState::Up;
    }

    /// updates all keys released in last frame to KeyState::NotPressed
    /// and all keys pressed in last frame to KeyState::Pressed
    pub(crate) fn update_key_state(&mut self) {
        self.keys.iter_mut().for_each(|k| {
            *k = match *k {
                KeyState::Up => KeyState::NotPressed,
                KeyState::Down => KeyState::Pressed,
                x => x,
            };
        });
    }

    /// Returns true during the frame the user starts pressing down the key identified by KeyCode.
    pub fn get_key_down(&self, key: KeyCode) -> bool {
        match self.keys[key as usize] {
            KeyState::Down => true,
            _ => false,
        }
    }

    /// Returns true during the frame the user releases the key identified by KeyCode.
    pub fn get_key_up(&self, key: KeyCode) -> bool {
        match self.keys[key as usize] {
            KeyState::Up => true,
            _ => false,
        }
    }

    /// Returns true while the user holds down the key identified by KeyCode.
    pub fn get_key(&self, key: KeyCode) -> bool {
        match self.keys[key as usize] {
            KeyState::Pressed => true,
            _ => false,
        }
    }
}

impl Default for KeyBoard {
    fn default() -> Self {
        KeyBoard::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_states_are_updated() {
        let key = KeyCode::KeyA;
        let mut kb = KeyBoard::default();
        kb.press_key(key);
        assert_eq!(kb.get_key_down(key), true);
        assert_eq!(kb.get_key(key), false);
        kb.update_key_state();
        assert_eq!(kb.get_key_down(key), false);
        assert_eq!(kb.get_key(key), true);
        kb.release_key(key);
        assert_eq!(kb.get_key_up(key), true);
        assert_eq!(kb.get_key(key), false);
        kb.update_key_state();
        assert_eq!(kb.get_key_up(key), false);
        assert_eq!(kb.get_key(key), false);
    }
}
