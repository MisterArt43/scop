use std::ffi::c_void;

use gl::{
    self,
    types::{GLenum, GLuint},
};

#[derive(Debug, Clone, Copy)]
pub enum BufferUsage {
    Static,
    Dynamic,
    Stream,
}

impl BufferUsage {
    pub fn to_gl(self) -> GLenum {
        match self {
            Self::Static => gl::STATIC_DRAW,
            Self::Dynamic => gl::DYNAMIC_DRAW,
            Self::Stream => gl::STREAM_DRAW,
        }
    }
}

#[derive(Debug)]
pub struct VBO {
    id: GLuint,
}

impl VBO {
    pub fn new() -> Self {
        let mut id = 0;

        unsafe {
            gl::GenBuffers(1, &mut id);
        }

        Self { id }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }

    pub fn unbind() {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn upload<T>(&self, data: &[T], usage: BufferUsage) {
        self.bind();

        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                std::mem::size_of_val(data) as isize,
                data.as_ptr() as *const c_void,
                usage.to_gl(),
            );
        }

        Self::unbind();
    }

    pub fn id(&self) -> GLuint {
        self.id
    }
}

impl Drop for VBO {
    fn drop(&mut self) {
        if self.id != 0 {
            unsafe {
                gl::DeleteBuffers(1, &self.id);
            }
        }
    }
}
