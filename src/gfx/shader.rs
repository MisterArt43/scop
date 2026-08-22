use anyhow::{Result, anyhow};
use std::{cell::RefCell, collections::HashMap, ffi::CString, fs::read, ptr::null_mut};

use gl::{
    AttachShader, COMPILE_STATUS, CompileShader, CreateProgram, CreateShader, DeleteShader, FALSE, FRAGMENT_SHADER, GetProgramInfoLog, GetProgramiv, GetShaderInfoLog, GetShaderiv, GetUniformLocation, INFO_LOG_LENGTH, LINK_STATUS, LinkProgram, ShaderSource, Uniform1f, Uniform1i, Uniform2fv, Uniform3fv, Uniform4fv, UniformMatrix4fv, UseProgram, VERTEX_SHADER, types::{self, GLint, GLuint},
};

#[derive(Default)]
pub struct Shader {
    pub id: GLuint,
    uniforms: RefCell<HashMap<String, GLint>>,
}

impl Shader {
    /*================== PRIVATE =================*/

    fn check_shader_compile(shader: GLuint) -> Result<()> {
        let mut success: GLint = 0;
        let mut log_lenght: GLint = 0;
        unsafe {
            GetShaderiv(shader, INFO_LOG_LENGTH, &mut log_lenght);
            GetShaderiv(shader, COMPILE_STATUS, &mut success);
            if success == FALSE as i32 {
                let mut info_log: Vec<u8> = vec![0; log_lenght as usize];
                GetShaderInfoLog(shader, log_lenght, null_mut(), info_log.as_mut_ptr() as *mut i8);
                return Err(anyhow!(
                    "SHADER_COMPILATION_ERROR:\n {}",
                    String::from_utf8(info_log.to_vec())?
                ));
            }
        }
        Ok(())
    }

    fn check_program_link(program: GLuint) -> Result<()> {
        let mut success: GLint = 0;
        let mut log_lenght: GLint = 0;
        unsafe {
            GetProgramiv(program, LINK_STATUS, &mut success);
            if success == FALSE as i32 {
                GetProgramiv(program, INFO_LOG_LENGTH, &mut log_lenght);
                let mut info_log: Vec<u8> = vec![0; log_lenght as usize];
                GetProgramInfoLog(program, log_lenght, null_mut(), info_log.as_mut_ptr() as *mut i8);
                return Err(anyhow!(
                    "SHADER_LINKING_ERROR:\n {}",
                    String::from_utf8(info_log.to_vec())?
                ));
            }
        }
        Ok(())
    }

    fn get_uniform_location(&self, name: &str) -> GLint {
        if let Some(&location) = self.uniforms.borrow().get(name) {
            return location;
        }

        let c_name = CString::new(name)
            .expect("Failed to convert uniform name to CString");

        let location =
            unsafe { GetUniformLocation(self.id, c_name.as_ptr()) };

        self.uniforms
            .borrow_mut()
            .insert(name.to_owned(), location);

        location
    }

    /*================== PUBLIC =================*/

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
            Shader::check_shader_compile(vert_shader)?;

            let frag_shader = CreateShader(FRAGMENT_SHADER);

            let f_ptr = fragment_str.as_ptr() as *const i8;
            let f_len: GLint = fragment_str.len().try_into()?;
            ShaderSource(frag_shader, 1, &f_ptr, &f_len);
            CompileShader(frag_shader);
            Shader::check_shader_compile(frag_shader)?;

            shader.id = CreateProgram();
            AttachShader(shader.id, vert_shader);
            AttachShader(shader.id, frag_shader);

            LinkProgram(shader.id);
            Shader::check_program_link(shader.id)?;

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

    pub fn set_uniform_int(&self, name: &str, value: types::GLint) {
        unsafe {
            let location = self.get_uniform_location(name);
            Uniform1i(location, value);
        }
    }

    pub fn set_uniform_float(&self, name: &str, value: types::GLfloat) {
        unsafe {
            let location = self.get_uniform_location(name);
            Uniform1f(location, value);
        }
    }

    pub fn set_uniform_bool(&self, name: &str, value: bool) {
        unsafe {
            let location = self.get_uniform_location(name);
            Uniform1i(location, if value { 1 } else { 0 });
        }
    }

    pub fn set_uniform_vec2(&self, name: &str, value: &[types::GLfloat; 2]) {
        unsafe {
            let location = self.get_uniform_location(name);
            Uniform2fv(location, 1, value.as_ptr());
        }
    }

    pub fn set_uniform_vec3(&self, name: &str, value: &[types::GLfloat; 3]) {
        unsafe {
            let location = self.get_uniform_location(name);
            Uniform3fv(location, 1, value.as_ptr());
        }
    }

    pub fn set_uniform_vec4(&self, name: &str, value: &[types::GLfloat; 4]) {
        unsafe {
            let location = self.get_uniform_location(name);
            Uniform4fv(location, 1, value.as_ptr());
        }
    }

    pub fn set_uniform_mat4(&self, name: &str, value: &[types::GLfloat; 16]) {
        unsafe {
            let location = self.get_uniform_location(name);
            UniformMatrix4fv(location, 1, gl::FALSE, value.as_ptr());
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        if self.id != 0 {
            unsafe {
                gl::DeleteProgram(self.id);
            }
        }
    }
}