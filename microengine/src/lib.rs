//! microengine is a project developed during a computer graphics course on University of Wrocław.
//! It contains some boilerplate code which helped me simplify work on tasks during
//! It structure is heavily inspired by my experience with Unity game engine
//! and also by ggez rust crate
//! [!WARNING]
//! This framework is not thread safe! Do not use in multithreaded applications.
extern crate gl;
extern crate glfw;

pub mod context;
pub mod error;
mod event_handler;
pub mod game;
pub mod gameobject;
mod input;
pub mod scene;
mod timer;
mod window;

pub use game::{Game, GameConfig};
pub use gameobject::{GameObject, GameObjectId};
pub use input::keyboard::keys::KeyCode;
pub use window::WindowConfig;

// TODO: better document each module
// TODO: test game module with mock backend
