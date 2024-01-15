use gl_utils::{CompiledProgram, Texture, primitives};
use crate::gameobjects::light_proxy::LIGHT_BUFFER_BINDING_POINT;
use crate::config::debug;


const VERT_SHADER_PATH: &str = "./examples/volcano/gameobjects/volcano/shaders/volcano.vs";
const FRAG_SHADER_PATH: &str = "./examples/volcano/gameobjects/volcano/shaders/volcano.fs";
const TEXTURE_PATH: &str = "./examples/volcano/assets/fuji.png";

pub struct VolcanoMesh{
    program: CompiledProgram,
    texture: Texture,
    indices: usize,
}

impl VolcanoMesh {

    pub fn new() -> Self {
        let volcano = primitives::SolidOfRevolution::new(2.0, 30, 15, false, |x| (x.powi(2) + 1.0f32).sqrt());
        let t = Self {
            program: CompiledProgram::new(VERT_SHADER_PATH, FRAG_SHADER_PATH),
            texture: Texture::from(TEXTURE_PATH),
            indices: volcano.indices.len(),
        };
        t.set_buffers(volcano);
        t
    }

    fn set_buffers(&self, volcano: primitives::SolidOfRevolution) {
        
        let buffer = volcano.verts.iter()
            .zip(volcano.normals.iter())
            .flat_map(|(v, n)| vec![v.x, v.y, v.z, n.x, n.y, n.z])
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
                (volcano.indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                volcano.indices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );

            // vertex data
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
                (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
            );
        }
    }

    pub fn draw(&self, camera_pos: &glm::Vec3, projection: &glm::Mat4, model: &glm::Mat4, time: f32) {
        self.program.bind_program();
        self.program.bind_vao();
        self.texture.bind_texture();
        self.program.bind_uniform_to_block_idx("LightData", LIGHT_BUFFER_BINDING_POINT);
        unsafe {
            gl::Disable(gl::CULL_FACE); 
            if debug() {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            }
            gl::Uniform1f(
                self.program.get_uniform_location("time"),
                time,
            );
            gl::UniformMatrix4fv( 
                self.program.get_uniform_location("projection"),
                1,
                gl::FALSE, 
                projection.iter().map(|&x| x).collect::<Vec<f32>>().as_ptr() as *const f32
            );
            gl::UniformMatrix4fv( 
                self.program.get_uniform_location("model"),
                1,
                gl::FALSE, 
                model.iter().map(|&x| x).collect::<Vec<f32>>().as_ptr() as *const f32
            );
            gl::Uniform3f(
                self.program.get_uniform_location("viewer_pos"),
                camera_pos.x,
                camera_pos.y,
                camera_pos.z
            );
            gl::DrawElements(
                gl::TRIANGLES,
                self.indices as i32,
                gl::UNSIGNED_INT, 
                0 as *const gl::types::GLvoid,
            );
            gl::Enable(gl::CULL_FACE); 
            if debug() {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            }
        }
    }
}

