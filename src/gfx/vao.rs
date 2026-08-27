use std::{
    ffi::c_void,
    mem::size_of,
};

use gl::{
    self, BindVertexArray, DeleteVertexArrays, EnableVertexAttribArray, VertexAttribIPointer, VertexAttribPointer, types::{GLenum, GLint, GLsizei, GLuint},
};

use crate::math::{vec2::Vec2, vec3::Vec3, vec4::Vec4};

use super::vbo::VBO;

pub trait VertexDataType {
    const GL_TYPE: GLenum;
    const COUNT: GLint;
    const INTEGER: bool;
}

impl VertexDataType for Vec2 {
    const GL_TYPE: GLenum = gl::FLOAT;
    const COUNT: GLint = 2;
    const INTEGER: bool = false;
}

impl VertexDataType for Vec3 {
    const GL_TYPE: GLenum = gl::FLOAT;
    const COUNT: GLint = 3;
    const INTEGER: bool = false;
}

impl VertexDataType for Vec4 {
    const GL_TYPE: GLenum = gl::FLOAT;
    const COUNT: GLint = 4;
    const INTEGER: bool = false;
}

#[derive(Debug, Clone, Copy)]
pub struct VertexAttribute {
    pub location: GLuint,
    pub count: GLint,
    pub data_type: GLenum,
    pub integer: bool,
    pub normalized: bool,
    pub offset: usize,
}

#[derive(Debug)]
pub struct VertexLayout {
    stride: GLsizei,
    attributes: Vec<VertexAttribute>,
}

/**------------------------------------------------------------------------
 **                           VertexLayout
 *? Décrit la disposition des données d'un sommet dans un tampon OpenGL.
 *? La classe stocke la taille d'un sommet ainsi que la liste de ses
 *? attributs, avec leur emplacement, leur nombre de composantes et leur
 *? décalage en mémoire.
 *
 *@param stride Taille totale d'un sommet, en octets.
 *------------------------------------------------------------------------**/
impl VertexLayout {
    pub fn new<T>() -> Self {
        Self {
            stride: size_of::<T>() as GLsizei,
            attributes: Vec::new(),
        }
    }

    pub fn push<T: VertexDataType>(mut self, offset: usize) -> Self {
        self.attributes.push(VertexAttribute {
            location: self.attributes.len() as GLuint,
            count: T::COUNT,
            data_type: T::GL_TYPE,
            integer: T::INTEGER,
            normalized: false,
            offset,
        });

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

    pub fn unbind() {
        unsafe {
            BindVertexArray(0);
        }
    }

    pub fn set_layout(&self, vbo: &VBO, layout: &VertexLayout) {
        self.bind();
        vbo.bind();

        for attribute in layout.attributes() {
            unsafe {
                EnableVertexAttribArray(attribute.location);
                
                if attribute.integer {
                    VertexAttribIPointer(
                        attribute.location,
                        attribute.count,
                        attribute.data_type,
                        layout.stride(),
                        attribute.offset as *const c_void,
                    );
                } else {
                    VertexAttribPointer(
                        attribute.location,
                        attribute.count,
                        attribute.data_type,
                        if attribute.normalized {
                            gl::TRUE
                        } else {
                            gl::FALSE
                        },
                        layout.stride(),
                        attribute.offset as *const c_void,
                    );
                }
            }
        }

        VBO::unbind();
        VAO::unbind();
    }
}

impl Drop for VAO {
    fn drop(&mut self) {
        if self.id != 0 {
            unsafe {
                DeleteVertexArrays(1, &self.id);
            }
        }
    }
}
