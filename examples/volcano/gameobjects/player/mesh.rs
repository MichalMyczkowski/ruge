use gl_utils::{CompiledProgram, Texture, primitives};
use crate::config::debug;


const VERT_SHADER_PATH: &str = "./examples/volcano/gameobjects/player/shaders/player.vs";
const FRAG_SHADER_PATH: &str = "./examples/volcano/gameobjects/player/shaders/player.fs";
const TEXTURE_PATH: &str = "./examples/3d_maze/assets/gradient.png";

pub struct PlayerMesh{
    program: CompiledProgram,
    texture: Texture,
    indices: usize,
}

impl PlayerMesh {

    pub fn new() -> Self {
        let cube = primitives::Cube::new();
        let t = Self {
            program: CompiledProgram::new(VERT_SHADER_PATH, FRAG_SHADER_PATH),
            texture: Texture::from(TEXTURE_PATH),
            indices: cube.indices.len(),
        };
        t.set_buffers(cube);
        t
    }

    fn set_buffers(&self, cube: primitives::Cube) {
        self.program.bind_buffers();
        let buffer = cube.verts.iter().zip(cube.texture_coordinates).flat_map(|(v, t)| vec![v.x, v.y, v.z, t.x, t.y]).collect::<Vec<f32>>();
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (buffer.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                buffer.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (cube.indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                cube.indices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
            // vertex data
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (5 * std::mem::size_of::<f32>()) as gl::types::GLint,
                std::ptr::null(),
            );
            // texture coordinates
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                (5 * std::mem::size_of::<f32>()) as gl::types::GLint,
                (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
            );
        }
    }

    pub fn draw(&self, mvp: glm::Mat4, tail_model: &glm::Mat4, blade1: &glm::Mat4, blade2: &glm::Mat4, time: f32) {
        self.program.bind_program();
        self.program.bind_vao();
        self.texture.bind_texture();
        unsafe {
            if debug() {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            }
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
            gl::UniformMatrix4fv( 
                self.program.get_uniform_location("tail_model"),
                1,
                gl::FALSE, 
                tail_model.iter().map(|&x| x).collect::<Vec<f32>>().as_ptr() as *const f32
            );
            gl::UniformMatrix4fv( 
                self.program.get_uniform_location("blade1"),
                1,
                gl::FALSE, 
                blade1.iter().map(|&x| x).collect::<Vec<f32>>().as_ptr() as *const f32
            );
            gl::UniformMatrix4fv( 
                self.program.get_uniform_location("blade2"),
                1,
                gl::FALSE, 
                blade2.iter().map(|&x| x).collect::<Vec<f32>>().as_ptr() as *const f32
            );
            gl::DrawElementsInstanced(
                gl::TRIANGLES,
                self.indices as i32,
                gl::UNSIGNED_INT, 
                0 as *const gl::types::GLvoid,
                4 as i32,
            );
            if debug() {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            }
        }
    }
}
