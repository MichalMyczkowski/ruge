use gl_utils::{CompiledProgram, Texture, primitives};

const VERT_SHADER_PATH: &str = "./examples/volcano/gameobjects/sun/shaders/sun.vs";
const FRAG_SHADER_PATH: &str = "./examples/volcano/gameobjects/sun/shaders/sun.fs";

pub struct SunMesh{
    program: CompiledProgram,
    radius: f32,
    indices: usize,
}

impl SunMesh {

    pub fn new(radius: f32) -> Self {
        let sphere = primitives::Sphere::new(23, 17);
        let t = Self {
            program: CompiledProgram::new(VERT_SHADER_PATH, FRAG_SHADER_PATH),
            radius,
            indices: sphere.indices.len(),
        };
        t.set_buffers(sphere);
        t
    }

    fn set_buffers(&self, sphere: primitives::Sphere) {
        self.program.bind_buffers();
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (sphere.verts.len() * 3 * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                sphere.verts.as_ptr() as *const gl::types::GLvoid,
                gl::DYNAMIC_DRAW,
            );
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (sphere.indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                sphere.indices.as_ptr() as *const gl::types::GLvoid,
                gl::DYNAMIC_DRAW,
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

    pub fn draw(&self, mvp: glm::Mat4, position: &glm::Vec3) {
        let mut center = glm::vec3_to_vec4(position);
        center.w = 1.0;
        center = mvp * center;
        self.program.bind_program();
        self.program.bind_vao();
        unsafe {
            gl::UniformMatrix4fv( 
                self.program.get_uniform_location("mvp"),
                1,
                gl::FALSE, 
                mvp.iter().map(|&x| x).collect::<Vec<f32>>().as_ptr() as *const f32
            );
            gl::Uniform1f(
                self.program.get_uniform_location("radius"),
                self.radius
            );
            gl::Uniform4f(
                self.program.get_uniform_location("sun_center"),
                center.x,
                center.y,
                center.z,
                center.w,
            );
            gl::DrawElements(
                gl::TRIANGLES,
                self.indices as i32,
                gl::UNSIGNED_INT, 
                0 as *const gl::types::GLvoid,
            );
        }
    }
}
