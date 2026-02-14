use std::{mem::offset_of, ptr::null};

use gl::{DrawElements, FLOAT, TRIANGLES, UNSIGNED_INT};

use crate::{ebo::EBO, vao::VAO, vbo::VBO};

pub struct Mesh {
    vao: VAO,
    vbo: VBO,
    ebo: EBO,
    index_count: usize,
}

pub struct Vertex {
    pub(crate) position: [f32; 3],
    pub(crate) normal: [f32; 3],
    pub(crate) uv: [f32; 2],
}

impl Mesh {
    pub fn new(vertices: &Vec<Vertex>, indices: &Vec<u32>) -> Mesh {
        let mesh = Mesh {
            vao: VAO::new(),
            vbo: VBO::new(vertices,
                (vertices.len() * std::mem::size_of::<Vertex>()) as isize,
            ),
            ebo: EBO::new(
                indices,
                (indices.len() * std::mem::size_of::<u32>()) as isize,
            ),
            index_count: indices.len(),
        };
        mesh.vao.bind();
        mesh.ebo.bind();

        mesh.vao.link_attrib(&mesh.vbo, 0, 3, FLOAT, size_of::<Vertex>() as i32, offset_of!(Vertex, position));
        mesh.vao.link_attrib(&mesh.vbo, 1, 3, FLOAT, size_of::<Vertex>() as i32, offset_of!(Vertex, normal));
        mesh.vao.link_attrib(&mesh.vbo, 2, 2, FLOAT, size_of::<Vertex>() as i32, offset_of!(Vertex, uv));
        
        mesh.vao.unbind();
        mesh.vbo.unbind();
        mesh.ebo.unbind();
        mesh
    }

    pub fn bind(&self) {
        self.vao.bind();
    }

    pub fn unbind(&self) {
        self.vao.unbind();
    }

    pub fn get_index_count(&self) -> usize {
        self.index_count
    }

    pub fn draw(&self) {
        self.vao.bind();
        unsafe {
            DrawElements(TRIANGLES, self.index_count as i32, UNSIGNED_INT, null());
        }
    }

    pub fn delete(&self) {
        self.vao.delete();
        self.vbo.delete();
        self.ebo.delete();
    }
}
