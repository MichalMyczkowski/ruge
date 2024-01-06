//! microengine is a project developed during a computer graphics course on University of Wrocław.
//! It contains some boilerplate code which helped me simplify work on tasks during
//! It structure is heavily inspired by my experience with Unity game engine
//! and also by ggez rust crate

// TODO make these optional as they are not used in core engine
extern crate gl;
extern crate glfw;
extern crate nalgebra_glm as glm;

pub mod components;
pub mod context;
pub mod error;
mod event_handler;
pub mod game;
pub mod gameobject;
mod input;
pub mod scene;
mod timer;
mod window;
pub mod prelude;

pub use event_handler::Backend;
pub use game::{Game, GameConfig};
pub use gameobject::{GameObject, GameObjectId};
pub use input::keyboard::keys::KeyCode;
pub use input::mouse::MouseButton;
pub use scene::Scene;
pub use window::WindowConfig;

// TODO: better document each module
// TODO: add fps upper limit
// TODO: scene transitioning
// TODO: Game.set_default_scene / set_starting_scene ? if not set then first added?
