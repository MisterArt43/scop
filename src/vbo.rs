use gl::{
    self, ARRAY_BUFFER, BindBuffer, BufferData, DeleteBuffers, GenBuffers, STATIC_DRAW,
    types::GLuint,
};

use crate::mesh::Vertex;

#[derive(Default, Debug, Clone)]
pub struct VBO {
    id: GLuint,
    has_color: bool,
}

impl VBO {
    pub fn new(vertices: &Vec<Vertex>, size: isize) -> VBO {
        // initialize vbo with ID undefined
        let mut vbo = VBO::default();
        vbo.has_color = vertices.iter().any(|v| v.color != [0.0; 3]);

        unsafe {
            GenBuffers(1, &mut vbo.id);
            BindBuffer(ARRAY_BUFFER, vbo.id);
            BufferData(
                ARRAY_BUFFER,
                size,
                vertices.as_ptr() as *const _,
                STATIC_DRAW,
            );
        }
        vbo
    }

    pub fn has_color(&self) -> bool {
        self.has_color
    }

    pub fn bind(&self) {
        unsafe {
            BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            BindBuffer(ARRAY_BUFFER, 0);
        }
    }

    pub fn delete(&self) {
        unsafe { DeleteBuffers(1, &self.id) }
    }
}
