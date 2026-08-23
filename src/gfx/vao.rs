use std::ffi::c_void;

use gl::{
    self, BindVertexArray, DeleteVertexArrays, EnableVertexAttribArray, FALSE, VertexAttribPointer,
    types::{GLenum, GLint, GLsizei, GLuint},
};

use super::vbo::VBO;

#[derive(Debug, Clone, Copy)]
pub struct VertexAttribute {
    pub location: GLuint,
    pub count: GLint,
    pub offset: usize,
}

#[derive(Debug)]
pub struct VertexLayout {
    stride: GLsizei,
    attributes: Vec<VertexAttribute>,
}

impl VertexLayout {
    pub fn new(stride: GLsizei) -> Self {
        Self {
            stride,
            attributes: Vec::new(),
        }
    }

    pub fn push(
        mut self,
        location: GLuint,
        count: GLint,
        offset: usize,
    ) -> Self {
        self.attributes.push(VertexAttribute { location, count, offset });

        self
    }

    pub fn stride(&self) -> GLsizei {
        self.stride
    }

    pub fn attributes(&self) -> &[VertexAttribute] {
        &self.attributes
    }
}

#[derive(Debug)]
pub struct VAO {
    id: GLuint,
}


impl VAO {
    pub fn new() -> VAO {
        // initialize vao with ID undefined
        let mut id = 0;

        unsafe { gl::GenVertexArrays(1, &mut id) }
        Self { id }
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

    /**=======================
     * todo      Deprecated
     *  changer la fonction par set_layout
     *  
     *========================**/
    pub fn link_attrib(
        &self,
        vbo: &VBO,
        layout: GLuint,
        num_components: GLint,
        type_: GLenum,
        stride: GLsizei,
        offset: usize,
    ) {
        vbo.bind();
        unsafe {
            VertexAttribPointer(
                layout,
                num_components,
                type_,
                FALSE,
                stride,
                offset as *const c_void,
            );
            EnableVertexAttribArray(layout);
        }
        VBO::unbind();
    }

    pub fn set_layout(
        &self,
        vbo: &VBO,
        layout: &VertexLayout,
    ) {
        self.bind();
        vbo.bind();

        for attribute in layout.attributes() {
            unsafe {
                EnableVertexAttribArray(attribute.location);

                VertexAttribPointer(
                    attribute.location,
                    attribute.count,
                    gl::FLOAT,
                    gl::FALSE,
                    layout.stride(),
                    attribute.offset as *const c_void,
                );
            }
        }
    }

    pub fn delete(&self) {
        unsafe { DeleteVertexArrays(1, &self.id) }
    }
}
