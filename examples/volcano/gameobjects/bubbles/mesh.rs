use gl_utils::{CompiledProgram, Texture, primitives};


const VERT_SHADER_PATH: &str = "./examples/volcano/gameobjects/bubbles/shaders/bubbles.vs";
const FRAG_SHADER_PATH: &str = "./examples/volcano/gameobjects/bubbles/shaders/bubbles.fs";
const TEXTURE_PATH: &str = "./examples/3d_maze/assets/gradient.png";

pub struct BubbleMesh{
    program: CompiledProgram,
    texture: Texture,
    radius: f32,
    indices: usize,
}

impl BubbleMesh {

    pub fn new(radius: f32) -> Self {
        let sphere = primitives::Sphere::new(radius, 23, 17);
        let t = Self {
            program: CompiledProgram::new(VERT_SHADER_PATH, FRAG_SHADER_PATH),
            texture: Texture::from(TEXTURE_PATH),
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
                (3 * sphere.verts.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                sphere.verts.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (sphere.indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                sphere.indices.as_ptr() as *const gl::types::GLvoid,
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
            gl::Enable(gl::CULL_FACE); 
        }
    }

    pub fn draw(&self, mvps: &Vec<f32>, colors: &Vec<f32>, time: f32, count: usize) {
        self.program.bind_program();
        self.program.bind_vao();
        self.texture.bind_texture();
        unsafe {
            gl::Uniform1f(
                self.program.get_uniform_location("time"),
                time,
            );
            gl::Uniform1f(
                self.program.get_uniform_location("radius"),
                self.radius,
            );
            gl::Uniform1fv( 
                self.program.get_uniform_location("colors"),
                count as i32,
                colors.as_ptr() as *const f32
            );
            gl::UniformMatrix4fv( 
                self.program.get_uniform_location("mvps"),
                count as i32,
                gl::FALSE, 
                mvps.as_ptr() as *const f32
            );
            gl::DrawElementsInstanced(
                gl::TRIANGLES,
                self.indices as i32,
                gl::UNSIGNED_INT, 
                0 as *const gl::types::GLvoid,
                count as i32,
            );
        }
    }
}
