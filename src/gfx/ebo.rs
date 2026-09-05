use std::ffi::c_void;

use gl::types::GLuint;

use super::BufferUsage;

#[derive(Debug)]
pub struct EBO {
    id: GLuint,
}

impl EBO {
    pub fn new() -> Self {
        let mut id = 0;

        unsafe {
            gl::GenBuffers(1, &mut id);
        }

        Self { id }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
        }
    }

    pub fn unbind() {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }

    pub fn upload(&self, indices: &[u32], usage: BufferUsage) {
        self.bind();

        unsafe {
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                std::mem::size_of_val(indices) as isize,
                indices.as_ptr() as *const c_void,
                usage.to_gl(),
            );
        }
    }

    pub fn id(&self) -> GLuint {
        self.id
    }
}

impl Drop for EBO {
    fn drop(&mut self) {
        if self.id != 0 {
            unsafe {
                gl::DeleteBuffers(1, &self.id);
            }
        }
    }
}
