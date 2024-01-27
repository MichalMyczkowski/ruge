use gl_utils::{CompiledProgram, Texture, primitives};
use microengine::components::transform::Transform;
use crate::config::debug;

const VERT_SHADER_PATH: &str = "./examples/earth/gameobjects/earth/shaders/chunk.vs";
const FRAG_SHADER_PATH: &str = "./examples/earth/gameobjects/earth/shaders/chunk.fs";

pub struct ChunkMesh {
    program: CompiledProgram,
    indices: usize,
    radius: f32,
}

impl ChunkMesh {

    pub fn new(transform: &mut Transform, radius: f32) -> Self {
        let ground = primitives::Plane::new(20);
        let t = Self {
            program: CompiledProgram::new(VERT_SHADER_PATH, FRAG_SHADER_PATH),
            indices: ground.indices.len(),
            radius,
        };
        t.set_buffers(ground, transform);
        t
    }

    fn set_buffers(&self, ground: primitives::Plane, transform: &mut Transform) {
        let buffer = ground.verts.iter()
            .zip(ground.texture_coordinates.iter())
            .flat_map(|(vert, tex_coord)| { 
                let mut vert = transform.local_to_world() * glm::vec3_to_vec4(&vert);
                vert += glm::vec3_to_vec4(transform.position());
                vec![vert.x, vert.y, vert.z, tex_coord.x, tex_coord.y]
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

    pub fn draw(&self, projection: &glm::Mat4, start_pos: &glm::Vec3, scale_vec: &glm::Vec3, mix: f32) {
        unsafe {
            gl::Uniform3f(
                self.program.get_uniform_location("start_pos"),
                start_pos.x,
                start_pos.y,
                start_pos.z,
            );
            gl::Uniform3f(
                self.program.get_uniform_location("scale_vec"),
                scale_vec.x,
                scale_vec.y,
                scale_vec.z,
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
