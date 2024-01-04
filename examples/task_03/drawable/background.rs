use gl;
use gl_utils::{CompiledProgram, Texture};
use glm::Vec2;

const VERT_SHADER_PATH: &str = "./examples/task_03/shaders/background.vs";
const FRAG_SHADER_PATH: &str = "./examples/task_03/shaders/background.fs";
const TEXTURE_PATH: &str = "./examples/task_03/assets/background.png";

pub struct BackgroundDrawable {
    program: CompiledProgram,
    texture: Texture,
    verts: Vec<Vec2>,
    indices: Vec<u32>,
    pub position: Vec2,
    pub rotation: f32,
}

impl BackgroundDrawable {
    fn calculate_verts_and_indices(size: f32) -> (Vec<Vec2>, Vec<u32>) {
        let half_size = size / 2.0;
        // layout:
        // position | texture coordinate
        (
            vec![
                Vec2::new(-half_size, half_size),
                Vec2::new(0.0, 1.0),
                Vec2::new(half_size, half_size),
                Vec2::new(1.0, 1.0),
                Vec2::new(-half_size, -half_size),
                Vec2::new(0.0, 0.0),
                Vec2::new(half_size, -half_size),
                Vec2::new(1.0, 0.0),
            ],
            vec![2, 1, 0, 2, 3, 1],
        )
    }

    pub fn new(size: f32, position: Vec2) -> Self {
        let (verts, indices) = Self::calculate_verts_and_indices(size);
        let t = BackgroundDrawable {
            program: CompiledProgram::new(VERT_SHADER_PATH, FRAG_SHADER_PATH),
            texture: Texture::from(TEXTURE_PATH),
            position,
            verts,
            indices,
            rotation: 0f32,
        };
        t.set_buffers();
        t
    }

    fn set_buffers(&self) {
        self.program.bind_buffers();
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.verts.len() * 2 * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                self.verts.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (self.indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                self.indices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                2,
                gl::FLOAT,
                gl::FALSE,
                (4 * std::mem::size_of::<f32>()) as gl::types::GLint,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                (4 * std::mem::size_of::<f32>()) as gl::types::GLint,
                (2 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
            );
        }
    }

    pub fn draw(&self, aspect_ratio: f32, time: f32) {
        self.program.bind_program();
        self.program.bind_vao();
        self.texture.bind_texture();
        unsafe {
            gl::Uniform1f(self.program.get_uniform_location("rotation"), self.rotation);
            gl::Uniform1f(self.program.get_uniform_location("time"), time);
            gl::Uniform1f(
                self.program.get_uniform_location("aspect_ratio"),
                aspect_ratio,
            );
            gl::Uniform2f(
                self.program.get_uniform_location("position"),
                self.position.x,
                self.position.y,
            );
            gl::DrawElements(
                gl::TRIANGLES,
                self.indices.len() as i32,
                gl::UNSIGNED_INT,
                0 as *const gl::types::GLvoid,
            );
        }
    }
}
