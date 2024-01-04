use crate::utils::Color;
use gl;
use gl_utils::CompiledProgram;
use glm::Vec2;

const VERT_SHADER_PATH: &str = "./examples/task_03/shaders/triangle.vs";
const FRAG_SHADER_PATH: &str = "./examples/task_03/shaders/triangle.fs";

pub enum TriangleType {
    Equilateral { height: f32 },
    Isosceles { height: f32, base: f32 },
}
use TriangleType::*;

impl TriangleType {
    fn calculate_verts(&self) -> Vec<Vec2> {
        match self {
            Equilateral { height } => {
                vec![
                    Vec2::new(height / (-3f32), height / 3.0 * (3f32).sqrt()),
                    Vec2::new(height / (-3f32), -height / 3.0 * (3f32).sqrt()),
                    Vec2::new(height / (3f32) * 2f32, 0f32),
                ]
            }
            Isosceles { height, base } => {
                vec![
                    Vec2::new(height / (-3f32), base / 2.0),
                    Vec2::new(height / (-3f32), -base / 2.0),
                    Vec2::new(height / (3f32) * 2f32, 0f32),
                ]
            }
        }
    }
}

pub struct Triangle {
    program: CompiledProgram,
    pub verts: Vec<Vec2>,
    pub color: Color,
    pub position: Vec2,
    pub rotation: f32,
}

impl Triangle {
    pub fn new(tri_type: TriangleType, position: Vec2, color: Color) -> Self {
        let t = Triangle {
            program: CompiledProgram::new(VERT_SHADER_PATH, FRAG_SHADER_PATH),
            color,
            position,
            verts: tri_type.calculate_verts(),
            rotation: 0f32,
        };
        t.set_buffers();
        t
    }

    fn set_buffers(&self) {
        self.program.bind_buffers();
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.verts.len() * 2 * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                self.verts.as_ptr() as *const gl::types::GLvoid,
                gl::DYNAMIC_DRAW,
            );
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                2,
                gl::FLOAT,
                gl::FALSE,
                (2 * std::mem::size_of::<f32>()) as gl::types::GLint,
                std::ptr::null(),
            );
        }
    }

    pub fn draw(&self, aspect_ratio: f32) {
        self.program.bind_program();
        self.program.bind_vao();
        unsafe {
            gl::Uniform1f(self.program.get_uniform_location("rotation"), self.rotation);
            gl::Uniform1f(
                self.program.get_uniform_location("aspect_ratio"),
                aspect_ratio,
            );
            gl::Uniform2f(
                self.program.get_uniform_location("position"),
                self.position.x,
                self.position.y,
            );
            gl::Uniform3f(
                self.program.get_uniform_location("color"),
                self.color.r,
                self.color.g,
                self.color.b,
            );
            gl::DrawArrays(gl::TRIANGLE_FAN, 0, self.verts.len() as i32);
        }
    }
}
