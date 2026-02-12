
use gl::{self, BindBuffer, BufferData, DeleteBuffers, ELEMENT_ARRAY_BUFFER, GenBuffers, STATIC_DRAW, types::GLuint};

#[derive(Default)]
pub struct EBO {
    id: GLuint
}

impl EBO {
    pub fn new(indices: &Vec<GLuint>, size: isize) -> EBO {
        // initialize ebo with ID undefined
        let mut ebo = EBO::default();

        unsafe {
            GenBuffers(1, &mut ebo.id);
            BindBuffer(ELEMENT_ARRAY_BUFFER, ebo.id);
            BufferData(ELEMENT_ARRAY_BUFFER, size, indices.as_ptr() as *const _, STATIC_DRAW);
        }
        ebo
    }

    pub fn bind(&self) {
        unsafe {
            BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            BindBuffer(ELEMENT_ARRAY_BUFFER, 0);
        }
    }

    pub fn delete(&self) {
        unsafe {
            DeleteBuffers(1, &self.id)
        }
    }
}