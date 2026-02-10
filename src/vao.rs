use std::{os::raw::c_void};

use gl::{self, BindVertexArray, DeleteVertexArrays, EnableVertexAttribArray, FALSE, VertexAttribPointer, types::{GLenum, GLint, GLsizei, GLuint}};

use crate::vbo::VBO;



#[derive(Default)]
pub struct VAO {
    id: GLuint
}

impl VAO {
    pub fn new() -> VAO {
        // initialize vao with ID undefined
        let mut vao = VAO::default();

        unsafe {
            gl::GenVertexArrays(1, &mut vao.id)
        }
        vao
    }

    pub fn bind(&self) {
        unsafe {
            BindVertexArray(self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            BindVertexArray(0);
        }
    }

    pub fn link_attrib(&self, vbo: &VBO, layout: GLuint, num_components: GLint, type_: GLenum, stride: GLsizei, offset: usize) {
        vbo.bind();
        unsafe {
            VertexAttribPointer(layout, num_components, type_, FALSE, stride, offset as *const c_void);
            EnableVertexAttribArray(layout);
        }
        vbo.unbind();
    } 

    pub fn delete(&self) {
        unsafe {
            DeleteVertexArrays(1, &self.id)
        }
    }
}