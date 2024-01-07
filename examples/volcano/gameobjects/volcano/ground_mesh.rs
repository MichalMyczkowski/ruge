use gl_utils::{CompiledProgram, Texture, primitives};
use crate::config::debug;


const VERT_SHADER_PATH: &str = "./examples/volcano/gameobjects/volcano/shaders/volcano.vs";
const FRAG_SHADER_PATH: &str = "./examples/volcano/gameobjects/volcano/shaders/volcano.fs";
const TEXTURE_PATH: &str = "./examples/volcano/assets/fuji.png";

pub struct GroundMesh{
    program: CompiledProgram,
    texture: Texture,
    indices: usize,
}

impl GroundMesh {

    pub fn new() -> Self {
        let ground = primitives::Plane::new(40);
        let t = Self {
            program: CompiledProgram::new(VERT_SHADER_PATH, FRAG_SHADER_PATH),
            texture: Texture::from(TEXTURE_PATH),
            indices: ground.indices.len(),
        };
        t.set_buffers(ground);
        t
    }

    fn set_buffers(&self, ground: primitives::Plane) {

        self.program.bind_buffers();
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (3 * ground.verts.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                ground.verts.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (ground.indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                ground.indices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );

            // vertex data
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (3 * std::mem::size_of::<f32>()) as gl::types::GLint,
                std::ptr::null(),
            );
        }
    }

    pub fn draw(&self, mvp: glm::Mat4, time: f32) {
        self.program.bind_program();
        self.program.bind_vao();
        self.texture.bind_texture();
        unsafe {
            if debug() {
            }
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            gl::Uniform1f(
                self.program.get_uniform_location("time"),
                time,
            );
            gl::UniformMatrix4fv( 
                self.program.get_uniform_location("mvp"),
                1,
                gl::FALSE, 
                mvp.iter().map(|&x| x).collect::<Vec<f32>>().as_ptr() as *const f32
            );
            gl::DrawElements(
                gl::TRIANGLES,
                self.indices as i32,
                gl::UNSIGNED_INT, 
                0 as *const gl::types::GLvoid,
            );
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            if debug() {
            }
        }
    }
}

