use std::rc::Weak;

use crate::utils::Color;
use gl;
use gl_utils::CompiledProgram;
use glm::Vec2;

const VERT_SHADER_PATH: &str = "./examples/task_03/shaders/labirynth.vs";
const FRAG_SHADER_PATH: &str = "./examples/task_03/shaders/labirynth.fs";

pub struct LabirynthDrawable {
    program: CompiledProgram,
    rotations: Weak<Vec<Vec<f32>>>,
    positions: Weak<Vec<Vec<Vec2>>>,
    instance_count: usize,
    pub color: Color,
    pub position: Vec2,
    pub rotation: f32,
    pub verts: Vec<Vec2>,
}

impl LabirynthDrawable {
    fn calculate_verts(height: f32) -> Vec<Vec2> {
        vec![
            Vec2::new(height / 3.0 * (3f32).sqrt() * 0.6, height / (-3f32)),
            Vec2::new(-height / 3.0 * (3f32).sqrt() * 0.6, height / (-3f32)),
            Vec2::new(0f32, height / (3f32) * 2f32),
        ]
    }

    pub fn new(
        height: f32,
        position: Vec2,
        color: Color,
        rotations: Weak<Vec<Vec<f32>>>,
        positions: Weak<Vec<Vec<Vec2>>>,
        instance_count: usize,
    ) -> Self {
        let t = Self {
            program: CompiledProgram::new(VERT_SHADER_PATH, FRAG_SHADER_PATH),
            color,
            position,
            rotations,
            positions,
            instance_count,
            verts: Self::calculate_verts(height),
            rotation: 0f32,
        };
        t.set_buffers();
        t
    }

    fn set_buffers(&self) {
        self.program.bind_buffers();
        let rotations = self.rotations.upgrade().unwrap();
        let positions = self.positions.upgrade().unwrap();
        let buffer = self
            .verts
            .iter()
            .flat_map(|v| [v.x, v.y])
            .chain(
                positions
                    .iter()
                    .flatten()
                    .zip(rotations.iter().flatten())
                    .flat_map(|(pos, &rot)| [pos.x, pos.y, rot]),
            )
            .collect::<Vec<f32>>();
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (buffer.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                buffer.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
            // vertex data
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                2,
                gl::FLOAT,
                gl::FALSE,
                (2 * std::mem::size_of::<f32>()) as gl::types::GLint,
                std::ptr::null(),
            );
            // position data
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                (3 * std::mem::size_of::<f32>()) as gl::types::GLint,
                (self.verts.len() * 2 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
            );

            gl::VertexAttribDivisor(1, 1);
            // rotation data
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(
                2,
                1,
                gl::FLOAT,
                gl::FALSE,
                (3 * std::mem::size_of::<f32>()) as gl::types::GLint,
                ((self.verts.len() * 2 + 2) * std::mem::size_of::<f32>())
                    as *const gl::types::GLvoid,
            );
            gl::VertexAttribDivisor(2, 1);
        }
    }

    pub fn draw(&self, aspect_ratio: f32) {
        self.program.bind_program();
        self.program.bind_vao();
        unsafe {
            gl::Uniform1f(
                self.program.get_uniform_location("aspect_ratio"),
                aspect_ratio,
            );
            gl::Uniform1i(
                self.program.get_uniform_location("last_instance"),
                (self.instance_count - 1) as i32,
            );
            gl::Uniform3f(
                self.program.get_uniform_location("color"),
                self.color.r,
                self.color.g,
                self.color.b,
            );
            gl::DrawArraysInstanced(
                gl::TRIANGLES,
                0,
                self.verts.len() as i32,
                self.instance_count as i32,
            );
        }
    }
}
