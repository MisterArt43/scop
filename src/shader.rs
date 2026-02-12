use anyhow::{Result, anyhow};
use std::{ffi::{CStr, CString}, fs::read, ptr::null_mut};

use gl::{
    AttachShader, COMPILE_STATUS, CompileShader, CreateProgram, CreateShader, DeleteProgram, DeleteShader, FALSE, FRAGMENT_SHADER, GetProgramInfoLog, GetProgramiv, GetShaderInfoLog, GetShaderiv, GetUniformLocation, LINK_STATUS, LinkProgram, ShaderSource, Uniform1f, Uniform1i, Uniform2fv, Uniform3fv, Uniform4fv, UniformMatrix4fv, UseProgram, VERTEX_SHADER, types::{self, GLint, GLuint}
};

#[derive(Default)]
pub struct Shader {
    id: GLuint,
}

impl Shader {
    pub fn new(vertex_file: &str, fragment_file: &str) -> Result<Shader> {
        let vertex_str = String::from_utf8(read(vertex_file)?)?;
        let fragment_str = String::from_utf8(read(fragment_file)?)?;
        let mut shader = Shader::default();

        unsafe {
            let vert_shader = CreateShader(VERTEX_SHADER);

            let v_ptr = vertex_str.as_ptr() as *const i8;
            let v_len: GLint = vertex_str.len().try_into()?;
            ShaderSource(vert_shader, 1, &v_ptr, &v_len);
            CompileShader(vert_shader);
            Shader::compile_error(vert_shader, "VERTEX")?;

            let frag_shader = CreateShader(FRAGMENT_SHADER);

            let f_ptr = fragment_str.as_ptr() as *const i8;
            let f_len: GLint = fragment_str.len().try_into()?;
            ShaderSource(frag_shader, 1, &f_ptr, &f_len);
            CompileShader(frag_shader);
            Shader::compile_error(frag_shader, "FRAGMENT")?;

            shader.id = CreateProgram();
            AttachShader(shader.id, vert_shader);
            AttachShader(shader.id, frag_shader);

            LinkProgram(shader.id);
            Shader::compile_error(shader.id, "PROGRAM")?;

            DeleteShader(vert_shader);
            DeleteShader(frag_shader);
        }

        Ok(shader)
    }

    pub fn activate(&self) {
        unsafe {
            UseProgram(self.id);
        }
    }

    pub fn delete(&self) {
        unsafe {
            DeleteProgram(self.id);
        }
    }

    fn compile_error(shader: GLuint, type_: &str) -> Result<()> {
        let mut has_compiled: GLint = 0;
        let mut info_log: [u8; 1024] = [0; 1024];
        match type_ {
            "PROGRAM" => unsafe {
                GetProgramiv(shader, LINK_STATUS, &mut has_compiled);
                if has_compiled == FALSE as i32 {
                    GetProgramInfoLog(shader, 1024, null_mut(), info_log.as_mut_ptr() as *mut i8);
                    Err(anyhow!(
                        "SHADER_LINKING_ERROR for: PROGRAM :\n {}",
                        String::from_utf8(info_log.to_vec())?
                    ))
                } else {
                    Ok(())
                }
            },
            _ => unsafe {
                GetShaderiv(shader, COMPILE_STATUS, &mut has_compiled);
                if has_compiled == FALSE as i32 {
                    GetShaderInfoLog(shader, 1024, null_mut(), info_log.as_mut_ptr() as *mut i8);
                    Err(anyhow!(
                        "SHADER_COMPILATION_ERROR for: {type_} :\n {}",
                        String::from_utf8(info_log.to_vec())?
                    ))
                } else {
                    Ok(())
                }
            },
        }
    }

    pub fn set_uniform_int(&self, name: &str, value: types::GLint) {
        unsafe {
            let location = GetUniformLocation(self.id, str_to_cstring(name).as_ptr());
            Uniform1i(location, value);
        }
    }

    pub fn set_uniform_float(&self, name: &str, value: types::GLfloat) {
        unsafe {
            let location = GetUniformLocation(self.id, str_to_cstring(name).as_ptr());
            Uniform1f(location, value);
        }
    }

    pub fn set_uniform_vec2(&self, name: &str, value: &[types::GLfloat; 2]) {
        unsafe {
            let location = GetUniformLocation(self.id, str_to_cstring(name).as_ptr());
            Uniform2fv(location, 1, value.as_ptr());
        }
    }

    pub fn set_uniform_vec3(&self, name: &str, value: &[types::GLfloat; 3]) {
        unsafe {
            let location = GetUniformLocation(self.id, str_to_cstring(name).as_ptr());
            Uniform3fv(location, 1, value.as_ptr());
        }
    }

    pub fn set_uniform_vec4(&self, name: &str, value: &[types::GLfloat; 4]) {
        unsafe {
            let location = GetUniformLocation(self.id, str_to_cstring(name).as_ptr());
            Uniform4fv(location, 1, value.as_ptr());
        }
    }

    pub fn set_uniform_mat4(&self, name: &str, value: &[types::GLfloat; 16]) {
        unsafe {
            let location = GetUniformLocation(self.id, str_to_cstring(name).as_ptr());
            UniformMatrix4fv(location, 1, gl::FALSE, value.as_ptr());
        }
    }
}

fn str_to_cstring(str:&str) -> CString {
    CString::new(str).expect("Failed to convert uniform name to CString")
}