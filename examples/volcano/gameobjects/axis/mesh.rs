use gl;
use gl_utils::CompiledProgram;

const VERT_SHADER_PATH: &str = "./examples/3d_maze/gameobjects/axis/shaders/axis.vs";
const FRAG_SHADER_PATH: &str = "./examples/3d_maze/gameobjects/axis/shaders/axis.fs";

pub struct AxisMesh {
    program: CompiledProgram,
    verts: Vec<f32>,
}

impl AxisMesh {
    fn calculate_verts() -> Vec<f32> {
        // layout:
        // position | colour
        vec![
            // first line (x)
            -10.0, 0.0, 0.0, /**/ 0.3, 0.0, 0.0, 1.0,
            10.0, 0.0, 0.0, /**/ 1.0, 0.0, 0.0, 1.0,
            // second line (y)
            0.0, -10.0, 0.0, /**/ 0.0, 0.3, 0.0, 1.0,
            0.0, 10.0, 0.0, /**/ 0.0, 1.0, 0.0, 1.0,
            // third line (z)
            0.0, 0.0, -10.0, /**/ 0.0, 0.0, 0.3, 1.0,
            0.0, 0.0, 10.0, /**/ 0.0, 0.0, 1.0, 1.0,
        ]
    }

    pub fn new() -> Self {
        let verts = Self::calculate_verts();
        let d = Self {
            program: CompiledProgram::new(VERT_SHADER_PATH, FRAG_SHADER_PATH),
            verts,
        };
        d.set_buffers();
        d
    }

    fn set_buffers(&self) {
        self.program.bind_buffers();
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.verts.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                self.verts.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (7 * std::mem::size_of::<f32>()) as gl::types::GLint,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                4,
                gl::FLOAT,
                gl::FALSE,
                (7 * std::mem::size_of::<f32>()) as gl::types::GLint,
                (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
            );
        }
    }

    pub fn draw(&self, transform_matrix: glm::Mat4, time: f32) {
        self.program.bind_program();
        self.program.bind_vao();
        unsafe {
            gl::UniformMatrix4fv( 
                self.program.get_uniform_location("mvp"),
                1, gl::FALSE, transform_matrix.iter().map(|&x| x).collect::<Vec<f32>>().as_ptr() as *const f32);
            gl::DrawArrays(
                gl::LINES,
                0,
                self.verts.len() as i32,
            );
        }
    }
}

