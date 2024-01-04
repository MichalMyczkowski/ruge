use std::iter;
use gl_utils::{CompiledProgram, Texture};
use microengine::components::transform::Transform;

const VERT_SHADER_PATH: &str = "./examples/3d_maze/gameobjects/maze/shaders/maze.vs";
const FRAG_SHADER_PATH: &str = "./examples/3d_maze/gameobjects/maze/shaders/maze.fs";
const TEXTURE_PATH: &str = "./examples/3d_maze/assets/texture.png";

pub struct MazeMesh{
    program: CompiledProgram,
    texture: Texture,
    indices: usize,
    instance_count: usize,
    size: usize,
}

impl MazeMesh {
    fn calculate_verts_and_indices(height: f32) -> (Vec<f32>, Vec<u32>) {
        let mut aux = 0u32;
        let H = (2f32 / 3f32) * height * 2f32.sqrt();
        let v_x = vec![-height / 3.0 * (3f32).sqrt(), 0.0, height / 3.0 * (3f32).sqrt(), 0.0];
        let v_z = vec![height / 3f32, -2f32 * height / 3f32, height / 3f32, 0.0];
        let v_y = vec![H / -3f32, H / -3f32, H / -3f32, H * 2f32/3f32];
        // layout:
        // position | texture coordinate
        (
            vec![
                // first tri
                v_x[2], v_y[2], v_z[2], /**/ 0.0, 1.0,
                v_x[1], v_y[1], v_z[1], /**/ 0.5, 0.0,
                v_x[0], v_y[0], v_z[0], /**/ 1.0, 1.0,
                // second tri
                v_x[1], v_y[1], v_z[1], /**/ 0.0, 1.0,
                v_x[3], v_y[3], v_z[3], /**/ 0.5, 0.0,
                v_x[0], v_y[0], v_z[0], /**/ 1.0, 1.0,
                // third tri
                v_x[2], v_y[2], v_z[2], /**/ 0.0, 1.0,
                v_x[3], v_y[3], v_z[3], /**/ 0.5, 0.0,
                v_x[1], v_y[1], v_z[1], /**/ 1.0, 1.0,
                // fourth tri
                v_x[0], v_y[0], v_z[0], /**/ 0.0, 1.0,
                v_x[3], v_y[3], v_z[3], /**/ 0.5, 0.0,
                v_x[2], v_y[2], v_z[2], /**/ 1.0, 1.0,
            ],
            iter::repeat_with(|| {
                let v = vec![0, 1, 2];
                let v = v.iter().map(|x| x + aux).collect::<Vec<u32>>();
                aux += 3;
                v
            }).take(4).flatten().collect::<Vec<u32>>(),
        )
    }

    pub fn new(
        height: f32,
        instance_count: usize,
        transforms: &mut Vec<Transform>,
        size: usize,
    ) -> Self {
        let (verts, indices) = Self::calculate_verts_and_indices(height);
        let t = Self {
            program: CompiledProgram::new(VERT_SHADER_PATH, FRAG_SHADER_PATH),
            instance_count,
            indices: indices.len(),
            texture: Texture::from(TEXTURE_PATH),
            size,
        };
        t.set_buffers(transforms, verts, indices);
        t
    }

    fn set_buffers(&self, transforms: &mut Vec<Transform>, verts: Vec<f32>, indices: Vec<u32>) {

        let model_matrices: Vec<f32> = transforms.iter_mut().flat_map(|t| {
            t.local_to_world().iter().map(|v| *v).collect::<Vec<f32>>()
        }).collect();

        let vert_len = verts.len();
        let buffer: Vec<f32> = verts.into_iter().chain(model_matrices).collect();
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
                (indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                indices.as_ptr() as *const gl::types::GLvoid,
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

            // model matrix 
            gl::EnableVertexAttribArray(2);
            gl::EnableVertexAttribArray(3);
            gl::EnableVertexAttribArray(4);
            gl::EnableVertexAttribArray(5);
            // 1st column
            gl::VertexAttribPointer(
                2,
                4,
                gl::FLOAT,
                gl::FALSE,
                (4 * 4 * std::mem::size_of::<f32>()) as gl::types::GLint,
                ((vert_len + 0) * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
            );
            // 2nd column
            gl::VertexAttribPointer(
                3,
                4,
                gl::FLOAT,
                gl::FALSE,
                (4 * 4 * std::mem::size_of::<f32>()) as gl::types::GLint,
                ((vert_len + 4) * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
            );
            // 3rd column
            gl::VertexAttribPointer(
                4,
                4,
                gl::FLOAT,
                gl::FALSE,
                (4 * 4 * std::mem::size_of::<f32>()) as gl::types::GLint,
                ((vert_len + 8) * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
            );
            // 4th column
            gl::VertexAttribPointer(
                5,
                4,
                gl::FLOAT,
                gl::FALSE,
                (4 * 4 * std::mem::size_of::<f32>()) as gl::types::GLint,
                ((vert_len + 12) * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
            );
            // split attribs by each instance
            gl::VertexAttribDivisor(2, 1);
            gl::VertexAttribDivisor(3, 1);
            gl::VertexAttribDivisor(4, 1);
            gl::VertexAttribDivisor(5, 1);
        }
    }

    pub fn draw(&self, projection: glm::Mat4, time: f32) {
        self.program.bind_program();
        self.program.bind_vao();
        self.texture.bind_texture();
        unsafe {
            gl::Uniform1i(
                self.program.get_uniform_location("last_instance"),
                (self.instance_count - 1) as i32,
            );
            gl::Uniform1i(
                self.program.get_uniform_location("size"),
                (self.size) as i32,
            );
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
            gl::DrawElementsInstanced(
                gl::TRIANGLES,
                self.indices as i32,
                gl::UNSIGNED_INT, 
                0 as *const gl::types::GLvoid,
                self.instance_count as i32
            );
        }
    }
}
