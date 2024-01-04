mod program;

use gl::types::*;
use program::{Program, Shader};
use std::fs::File;
use std::io::read_to_string;
use std::iter;

pub struct CompiledProgram {
    program: Program,
    vao_id: GLuint,
    vbo_id: GLuint,
    ebo_id: GLuint,
}

impl CompiledProgram {
    pub fn new(vs_path: &str, fs_path: &str) -> CompiledProgram {
        let program = CompiledProgram::compile_shaders(vs_path, fs_path);
        let mut vao_id: GLuint = 0;
        let mut vbo_id: GLuint = 0;
        let mut ebo_id: GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao_id);
            gl::BindVertexArray(vao_id);
            gl::GenBuffers(1, &mut ebo_id);
            gl::GenBuffers(1, &mut vbo_id);
        }
        CompiledProgram {
            program,
            vao_id,
            vbo_id,
            ebo_id,
        }
    }

    fn compile_shaders(vs_path: &str, fs_path: &str) -> Program {
        let vs = File::open(vs_path).expect(&format!("Missing vertex shader file: {}", vs_path));
        let fs = File::open(fs_path).expect(&format!("Missing fragment shader file: {}", fs_path));
        let vs = read_to_string(vs).unwrap();
        let fs = read_to_string(fs).unwrap();
        let vs = Shader::from_source(&vs, gl::VERTEX_SHADER).unwrap();
        let fs = Shader::from_source(&fs, gl::FRAGMENT_SHADER).unwrap();
        Program::from_shaders(&[vs, fs]).unwrap()
    }

    pub fn bind_program(&self) {
        self.program.set_used();
    }

    pub fn bind_buffers(&self) {
        unsafe {
            gl::BindVertexArray(self.vao_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo_id);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo_id);
        }
    }

    pub fn get_uniform_location(&self, name: &str) -> GLint {
        self.program.get_uniform_location(name)
    }

    pub fn bind_vao(&self) {
        unsafe {
            gl::BindVertexArray(self.vao_id);
        }
    }
}
impl Drop for CompiledProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vbo_id);
            gl::DeleteBuffers(1, &self.ebo_id);
            gl::DeleteVertexArrays(1, &self.vao_id);
        }
    }
}
