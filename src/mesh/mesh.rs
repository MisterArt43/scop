use std::{
    mem::offset_of,
    ptr::null,
};

use gl::{DrawElements, TRIANGLES, UNSIGNED_INT};

use crate::{
    gfx::{
        BufferUsage,
        VertexLayout,
    },
    math::{
        vec2::Vec2,
        vec3::Vec3,
    },
    mesh::Vertex,
    ebo::EBO,
    vao::VAO,
    vbo::VBO,
};

#[derive(Debug)]
pub struct Mesh {
    pub vao: VAO,
    pub vbo: VBO,
    pub ebo: EBO,

    pub nb_vertices: usize,
    pub index_count: usize,

    pub bbox_min: [f32; 3],
    pub bbox_max: [f32; 3],
}

impl Mesh {
    pub fn new(vertices: &[Vertex], indices: &[u32]) -> Self {
        let (bbox_min, bbox_max) = Self::compute_bbox(vertices);

        /*
         * VBO
         */
        let vbo = VBO::new();
        vbo.upload(vertices, BufferUsage::Static);

        /*
         * VAO
         */
        let vao = VAO::new();

        let layout = VertexLayout::new::<Vertex>()
            .push::<Vec3>(offset_of!(Vertex, position))
            .push::<Vec3>(offset_of!(Vertex, normal))
            .push::<Vec2>(offset_of!(Vertex, uv))
            .push::<Vec3>(offset_of!(Vertex, color));

        vao.set_layout(&vbo, &layout);

        /*
         * EBO
         *
         * ELEMENT_ARRAY_BUFFER fait partie de l'état du VAO,
         * donc on bind le VAO avant l'EBO.
         */
        let ebo = EBO::new();

        vao.bind();
        ebo.upload(indices, BufferUsage::Static);
        VAO::unbind();

        Self {
            vao,
            vbo,
            ebo,

            nb_vertices: vertices.len(),
            index_count: indices.len(),

            bbox_min,
            bbox_max,
        }
    }

    pub fn draw(&self) {
        self.vao.bind();

        unsafe {
            DrawElements(
                TRIANGLES,
                self.index_count as i32,
                UNSIGNED_INT,
                null(),
            );
        }
    }

    fn compute_bbox(vertices: &[Vertex]) -> ([f32; 3], [f32; 3]) {
        if vertices.is_empty() {
            return ([0.0; 3], [0.0; 3]);
        }

        let mut min = [f32::INFINITY; 3];
        let mut max = [f32::NEG_INFINITY; 3];

        for vertex in vertices {
            for i in 0..3 {
                min[i] = min[i].min(vertex.position[i]);
                max[i] = max[i].max(vertex.position[i]);
            }
        }

        (min, max)
    }

    pub fn bind(&self) {
        self.vao.bind();
    }

    pub fn unbind() {
        VAO::unbind();
    }

    pub fn vertex_count(&self) -> usize {
        self.nb_vertices
    }

    pub fn index_count(&self) -> usize {
        self.index_count
    }
}