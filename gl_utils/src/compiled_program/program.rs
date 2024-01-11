mod shader;
mod utils;
use gl::types::*;
pub use shader::Shader;
use std::ffi::CString;
use utils::create_whitespace_cstring_with_len;

pub struct Program {
    id: GLuint,
}

impl Program {
    pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
        let program_id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe {
                gl::AttachShader(program_id, shader.id());
            }
        }

        unsafe {
            gl::LinkProgram(program_id);
        }

        let mut success: GLint = 1;
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: GLint = 0;
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut GLchar,
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe {
                gl::DetachShader(program_id, shader.id());
            }
        }

        Ok(Program { id: program_id })
    }

    pub fn id(&self) -> GLuint {
        self.id
    }

    pub fn get_uniform_location(&self, uniform_name: &str) -> GLint {
        let mut loc: GLint = 0;
        unsafe {
            loc =
                gl::GetUniformLocation(self.id(), (&CString::new(uniform_name).unwrap()).as_ptr());
        }
        loc
    }

    pub fn bind_uniform_to_block_idx(&self, name: &str, block_idx: u32) {
        let mut loc: GLuint = 0;
        unsafe {
            loc = gl::GetUniformBlockIndex(self.id(), (&CString::new(name).unwrap()).as_ptr());
            gl::UniformBlockBinding(self.id(), loc, block_idx);
        }
    }

    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
