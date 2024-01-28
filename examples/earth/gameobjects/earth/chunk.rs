use gl_utils::{CompiledProgram, Texture, primitives};
use microengine::components::transform::Transform;
use crate::config::debug;

const VERT_SHADER_PATH: &str = "./examples/earth/gameobjects/earth/shaders/chunk.vs";
const FRAG_SHADER_PATH: &str = "./examples/earth/gameobjects/earth/shaders/chunk.fs";

pub struct ChunkMesh {
    program: CompiledProgram,
    indices: usize,
    radius: f32,
    up: glm::Vec3,
    right: glm::Vec3,
    clr: glm::Vec3,
}

impl ChunkMesh {

    pub fn new(up: glm::Vec3, right: glm::Vec3, clr: glm::Vec3, radius: f32) -> Self {
        let ground = primitives::Plane::new(2);
        let t = Self {
            program: CompiledProgram::new(VERT_SHADER_PATH, FRAG_SHADER_PATH),
            indices: ground.indices.len(),
            radius,
            up,
            right,
            clr,
        };
        t.set_buffers(ground);
        t
    }

    fn set_buffers(&self, ground: primitives::Plane) {
        let buffer = ground.verts.iter()
            .zip(ground.texture_coordinates.iter())
            .flat_map(|(vert, tex_coord)| { 
                vec![(vert.x + 1.0) * 0.5, vert.y, (vert.z + 1.0) * 0.5, tex_coord.x, tex_coord.y]
            })
            .collect::<Vec<f32>>();
        self.program.bind_buffers();
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (buffer.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                buffer.as_ptr() as *const gl::types::GLvoid,
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
                (5 * std::mem::size_of::<f32>()) as gl::types::GLint,
                std::ptr::null(),
            );
            // texture coordinates data
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

    pub fn bind_shader(&self) {
        self.program.bind_program();
        self.program.bind_vao();
    }

    pub fn draw(&self, projection: &glm::Mat4, start_pos: &glm::Vec3, mix: f32, width: f32) {
        unsafe {
            gl::Uniform3f(
                self.program.get_uniform_location("start_pos"),
                start_pos.x,
                start_pos.y,
                start_pos.z,
            );
            gl::Uniform3f(
                self.program.get_uniform_location("up"),
                self.up.x,
                self.up.y,
                self.up.z,
            );
            gl::Uniform3f(
                self.program.get_uniform_location("right"),
                self.right.x,
                self.right.y,
                self.right.z,
            );
            gl::Uniform3f(
                self.program.get_uniform_location("clr"),
                self.clr.x,
                self.clr.y,
                self.clr.z,
            );
            gl::Uniform1f(
                self.program.get_uniform_location("width"),
                width,
            );
            gl::Uniform1f(
                self.program.get_uniform_location("mix_val"),
                mix,
            );
            gl::Uniform1f(
                self.program.get_uniform_location("radius"),
                self.radius,
            );
            gl::Uniform1i(
                self.program.get_uniform_location("full"),
                1,
            );
            gl::UniformMatrix4fv( 
                self.program.get_uniform_location("projection"),
                1,
                gl::FALSE, 
                projection.iter().map(|&x| x).collect::<Vec<f32>>().as_ptr() as *const f32
            );
            gl::DrawElements(
                gl::TRIANGLES,
                self.indices as i32,
                gl::UNSIGNED_INT, 
                0 as *const gl::types::GLvoid,
            );
            if debug() {
                gl::Uniform1i(
                    self.program.get_uniform_location("full"),
                    0,
                );
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
                gl::DrawElements(
                    gl::TRIANGLES,
                    self.indices as i32,
                    gl::UNSIGNED_INT, 
                    0 as *const gl::types::GLvoid,
                );
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            }
        }
    }
}
