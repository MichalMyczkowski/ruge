//! This module provides Window struct which is passed with Context to gameobjects
//! and is used for reading and setting window related values
//! (setting full screen reading aspect ratio etc.)
//! and for closing game (ctx.window.close())

use std::cell::RefCell;

/// Initial window configuration
pub struct WindowConfig {
    pub name: String,
    pub width: usize,
    pub height: usize,
    pub posx: isize,
    pub posy: isize,
    pub is_fullscreen: bool,
}

impl WindowConfig {
    fn new(
        name: &str,
        width: usize,
        height: usize,
        posx: isize,
        posy: isize,
        is_fullscreen: bool,
    ) -> Self {
        WindowConfig {
            name: name.into(),
            width,
            height,
            posx,
            posy,
            is_fullscreen,
        }
    }
}

impl Default for WindowConfig {
    fn default() -> Self {
        WindowConfig::new("default", 1280, 720, 0, 0, false)
    }
}

pub struct Window {
    name: String,
    width: usize,
    height: usize,
    posx: isize,
    posy: isize,
    aspect_ratio: f64,
    is_fullscreen: bool,
    pub(crate) fullscreen_requested: RefCell<bool>,
    pub(crate) close_requested: RefCell<bool>,
    should_close: bool,
}

impl Window {
    pub fn new(
        name: &str,
        width: usize,
        height: usize,
        posx: isize,
        posy: isize,
        is_fullscreen: bool,
    ) -> Self {
        Window {
            name: String::from(name),
            should_close: false,
            close_requested: RefCell::new(false),
            fullscreen_requested: RefCell::new(false),
            is_fullscreen,
            width,
            height,
            posx,
            posy,
            aspect_ratio: (height as f64) / (width as f64),
        }
    }

    /// Use it in backend everytime window resizes
    pub(crate) fn system_update_resolution(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.aspect_ratio = (height as f64) / (width as f64);
    }

    /// Use it in backend implementations to indicate that window should close
    pub(crate) fn system_close(&mut self) {
        self.should_close = true;
    }

    /// Use it in backend to set window position on appropriate events
    pub(crate) fn system_set_pos(&mut self, posx: isize, posy: isize) {
        self.posx = posx;
        self.posy = posy;
    }

    /// Function indicates that either system requested to close window
    /// or user called window.close() and active scene has finished
    pub fn should_close(&self) -> bool {
        self.should_close
    }

    /// Set or unset fullscreen
    pub fn set_fullscreen(&self, fullscreen: bool) {
        *self.fullscreen_requested.borrow_mut() = fullscreen;
    }

    /// Use it to end the game.
    pub fn close(&self) {
        *self.close_requested.borrow_mut() = true;
    }

    /// Check if window is in fullscreen mode
    pub fn is_fullscreen(&self) -> bool {
        self.is_fullscreen
    }

    pub fn pos(&self) -> (isize, isize) {
        (self.posx, self.posy)
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.aspect_ratio
    }
}

impl Default for Window {
    fn default() -> Self {
        Window::new("default", 1280, 720, 0, 0, false)
    }
}

impl From<WindowConfig> for Window {
    fn from(value: WindowConfig) -> Self {
        Window {
            name: value.name,
            width: value.width,
            height: value.height,
            posx: value.posx,
            posy: value.posy,
            aspect_ratio: (value.height as f64) / (value.width as f64),
            is_fullscreen: value.is_fullscreen,
            ..Default::default()
        }
    }
}
