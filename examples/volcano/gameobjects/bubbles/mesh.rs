use gl_utils::{CompiledProgram, Texture, primitives};


const VERT_SHADER_PATH: &str = "./examples/volcano/gameobjects/bubbles/shaders/bubbles.vs";
const FRAG_SHADER_PATH: &str = "./examples/volcano/gameobjects/bubbles/shaders/bubbles.fs";
const TEXTURE_PATH: &str = "./examples/3d_maze/assets/gradient.png";

pub struct BubbleMesh{
    program: CompiledProgram,
    texture: Texture,
    radius: f32,
    indices: usize,
    sphere: primitives::Sphere,
}

impl BubbleMesh {

    pub fn new(radius: f32) -> Self {
        let sphere = primitives::Sphere::new(23, 17);
        let t = Self {
            program: CompiledProgram::new(VERT_SHADER_PATH, FRAG_SHADER_PATH),
            texture: Texture::from(TEXTURE_PATH),
            radius,
            indices: sphere.indices.len(),
            sphere,
        };
        t
    }

    fn update_buffers(&self, mvps: &Vec<f32>, colors: &Vec<f32>, count: usize) {
        let vert_len = self.sphere.verts.len() * 3;
        let buffer = self.sphere.verts
            .iter()
            .flat_map(|v| vec![v.x, v.y, v.z])
            .chain( mvps.iter().map(|x| *x).collect::<Vec<f32>>() )
            .chain( colors.iter().map(|x| *x).collect::<Vec<f32>>() )
            .collect::<Vec<f32>>();
        self.program.bind_buffers();
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (buffer.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                buffer.as_ptr() as *const gl::types::GLvoid,
                gl::DYNAMIC_DRAW,
            );
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (self.sphere.indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                self.sphere.indices.as_ptr() as *const gl::types::GLvoid,
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
            // model matrix 
            gl::EnableVertexAttribArray(1);
            gl::EnableVertexAttribArray(2);
            gl::EnableVertexAttribArray(3);
            gl::EnableVertexAttribArray(4);
            // 1st column
            gl::VertexAttribPointer(
                1,
                4,
                gl::FLOAT,
                gl::FALSE,
                (4 * 4 * std::mem::size_of::<f32>()) as gl::types::GLint,
                ((vert_len + 0) * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
            );
            // 2nd column
            gl::VertexAttribPointer(
                2,
                4,
                gl::FLOAT,
                gl::FALSE,
                (4 * 4 * std::mem::size_of::<f32>()) as gl::types::GLint,
                ((vert_len + 4) * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
            );
            // 3rd column
            gl::VertexAttribPointer(
                3,
                4,
                gl::FLOAT,
                gl::FALSE,
                (4 * 4 * std::mem::size_of::<f32>()) as gl::types::GLint,
                ((vert_len + 8) * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
            );
            // 4th column
            gl::VertexAttribPointer(
                4,
                4,
                gl::FLOAT,
                gl::FALSE,
                (4 * 4 * std::mem::size_of::<f32>()) as gl::types::GLint,
                ((vert_len + 12) * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
            );
            gl::EnableVertexAttribArray(5);
            // colors
            gl::VertexAttribPointer(
                5,
                1,
                gl::FLOAT,
                gl::FALSE,
                (1 * std::mem::size_of::<f32>()) as gl::types::GLint,
                ((vert_len + 16 * count) * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
            );
            // split attribs by each instance
            gl::VertexAttribDivisor(1, 1);
            gl::VertexAttribDivisor(2, 1);
            gl::VertexAttribDivisor(3, 1);
            gl::VertexAttribDivisor(4, 1);
            gl::VertexAttribDivisor(5, 1);
        }

    }

    // TODO: pass projection matrix as uniform and multiply in vertex shader instead
    pub fn draw(&self, mvps: &Vec<f32>, colors: &Vec<f32>, time: f32, count: usize) {
        self.update_buffers(mvps, colors, count);
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
