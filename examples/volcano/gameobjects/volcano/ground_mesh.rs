use gl_utils::{CompiledProgram, Texture, primitives};
use crate::config::debug;


const VERT_SHADER_PATH: &str = "./examples/volcano/gameobjects/volcano/shaders/ground.vs";
const FRAG_SHADER_PATH: &str = "./examples/volcano/gameobjects/volcano/shaders/ground.fs";
const NOISE_PATH: &str = "./examples/volcano/assets/perlin.png";

pub struct GroundMesh{
    program: CompiledProgram,
    noise: Texture,
    indices: usize,
    coord_offset: f32,
    vert_offset: f32,
}

impl GroundMesh {

    pub fn new() -> Self {
        let ground = primitives::Plane::new(40);
        let noise = Texture::from(NOISE_PATH);
        let t = Self {
            program: CompiledProgram::new(VERT_SHADER_PATH, FRAG_SHADER_PATH),
            noise,
            indices: ground.indices.len(),
            coord_offset: (ground.texture_coordinates[1].y - ground.texture_coordinates[0].y).abs(),
            vert_offset: (ground.verts[1].z - ground.verts[0].z).abs(),

        };
        t.set_buffers(ground);
        t
    }

    fn set_buffers(&self, ground: primitives::Plane) {
        let buffer = ground.verts.iter()
            .zip(ground.texture_coordinates.iter())
            .zip(ground.normals.iter())
            .flat_map(|((vert, tex_coord), normal)| 
                vec![vert.x, vert.y, vert.z, tex_coord.x, tex_coord.y, normal.x, normal.y, normal.z]
                )
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
                (8 * std::mem::size_of::<f32>()) as gl::types::GLint,
                std::ptr::null(),
            );
            // texture coordinates data
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                (8 * std::mem::size_of::<f32>()) as gl::types::GLint,
                (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
            );
            // normals data
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(
                2,
                3,
                gl::FLOAT,
                gl::FALSE,
                (8 * std::mem::size_of::<f32>()) as gl::types::GLint,
                (5 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
            );
        }
    }

    pub fn draw(&self, mvp: glm::Mat4, time: f32) {
        self.program.bind_program();
        self.program.bind_vao();
        self.noise.bind_texture();
        unsafe {
            gl::Uniform1f(
                self.program.get_uniform_location("time"),
                time,
            );
            gl::Uniform1f(
                self.program.get_uniform_location("coord_offset"),
                self.coord_offset,
            );
            gl::Uniform1f(
                self.program.get_uniform_location("vert_offset"),
                self.coord_offset,
            );
            gl::Uniform1i(
                self.program.get_uniform_location("full"),
                1,
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

