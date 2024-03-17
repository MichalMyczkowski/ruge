/// This package contains some basic, safe abstractions over OpenGL API
mod compiled_program;
mod texture;
pub mod primitives;
extern crate gl;
extern crate image;
extern crate nalgebra_glm as glm;

pub use compiled_program::CompiledProgram;
pub use texture::Texture;
