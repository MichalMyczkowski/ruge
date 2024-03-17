use gl_utils::{CompiledProgram, primitives};

const VERT_SHADER_PATH: &str = "./examples/rotating_cube/src/gameobjects/cube/shaders/cube.vs";
const FRAG_SHADER_PATH: &str = "./examples/rotating_cube/src/gameobjects/cube/shaders/cube.fs";

impl Default for CubeMesh {
    fn default() -> Self {
        CubeMesh::new()
    }
}

pub struct CubeMesh {
    program: CompiledProgram,
    indices: usize,
}

impl CubeMesh {

    pub fn new() -> Self {
        let cube = primitives::Cube::new();
        let t = Self {
            program: CompiledProgram::new(VERT_SHADER_PATH, FRAG_SHADER_PATH),
            indices: cube.indices.len(),
        };
        t.set_buffers(cube);
        t
    }

    fn set_buffers(&self, cube: primitives::Cube) {
        self.program.bind_buffers();
        let buffer = cube.verts.iter()
            .zip(cube.normals)
            .flat_map(|(v, n)| vec![v.x, v.y, v.z, n.x, n.y, n.z])
            .collect::<Vec<f32>>();
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
                (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
                std::ptr::null(),
            );
            // normal data
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

    pub fn draw(&self, camera_pos: &glm::Vec3, projection: &glm::Mat4, model: &glm::Mat4) {
        self.program.bind_program();
        self.program.bind_vao();
        unsafe {
            gl::UniformMatrix4fv( 
                self.program.get_uniform_location("projection"),
                1,
                gl::FALSE, 
                projection.iter().copied().collect::<Vec<f32>>().as_ptr() as *const f32
            );
            gl::UniformMatrix4fv( 
                self.program.get_uniform_location("model"),
                1,
                gl::FALSE, 
                model.iter().copied().collect::<Vec<f32>>().as_ptr() as *const f32
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
                std::ptr::null::<gl::types::GLvoid>(),
            );
        }
    }

}
