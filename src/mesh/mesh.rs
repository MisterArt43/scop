use crate::math::transform::{Transform};
use crate::mesh::{Vertex, parser};
use crate::{ebo::EBO, material::Mtl, vao::VAO, vbo::VBO};
use gl::{DrawElements, FLOAT, TRIANGLES, UNSIGNED_INT};
use std::mem::{offset_of, size_of};
use std::ptr::null;

#[derive(Debug, Clone)]
pub struct Mesh {
    pub vao: VAO,
    pub vbo: VBO,
    pub ebo: EBO,
    pub nb_vertices: usize,
    pub index_count: usize,
    pub bbox_min: [f32; 3],
    pub bbox_max: [f32; 3],
}

#[derive(Debug, Clone)]
pub struct SubMesh {
    pub object: String,
    pub group: String,
    pub material: String,
    pub material_data: Option<Mtl>,
    pub mesh: Mesh,
    pub transform: Transform,
}

pub struct Actor {
    pub submesh: Vec<SubMesh>,
    pub transform: Transform,
}

impl Actor {
    pub fn new(submesh: Vec<SubMesh>, transform: Option<Transform>) -> Actor {
        Actor {
            submesh,
            transform: transform.unwrap_or_else(Transform::new),
        }
    }
}

impl Mesh {
    pub fn new(vertices: &Vec<Vertex>, indices: &Vec<u32>) -> Mesh {
        let mut bbox_min = [std::f32::INFINITY; 3];
        let mut bbox_max = [std::f32::NEG_INFINITY; 3];
        for v in vertices {
            for i in 0..3 {
                if v.position[i] < bbox_min[i] {
                    bbox_min[i] = v.position[i];
                }
                if v.position[i] > bbox_max[i] {
                    bbox_max[i] = v.position[i];
                }
            }
        }

        let mesh = Mesh {
            vao: VAO::new(),
            vbo: VBO::new(vertices, (vertices.len() * size_of::<Vertex>()) as isize),
            ebo: EBO::new(indices, (indices.len() * size_of::<u32>()) as isize),
            nb_vertices: vertices.len(),
            index_count: indices.len(),
            bbox_min,
            bbox_max,
        };

        mesh.vao.bind();
        mesh.ebo.bind();

        mesh.vao.link_attrib(
            &mesh.vbo,
            0,
            3,
            FLOAT,
            size_of::<Vertex>() as i32,
            offset_of!(Vertex, position),
        );
        mesh.vao.link_attrib(
            &mesh.vbo,
            1,
            3,
            FLOAT,
            size_of::<Vertex>() as i32,
            offset_of!(Vertex, normal),
        );
        mesh.vao.link_attrib(
            &mesh.vbo,
            2,
            2,
            FLOAT,
            size_of::<Vertex>() as i32,
            offset_of!(Vertex, uv),
        );
        mesh.vao.link_attrib(
            &mesh.vbo,
            3,
            3,
            FLOAT,
            size_of::<Vertex>() as i32,
            offset_of!(Vertex, color),
        );

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

    pub fn from_obj(path: &str) -> Result<Vec<SubMesh>, String> {
        parser::parse_obj(path)
    }

    pub fn delete(&self) {
        self.vao.delete();
        self.vbo.delete();
        self.ebo.delete();
    }
}

impl SubMesh {
    pub fn new(object: String, group: String, material: String, material_data: Option<Mtl>, mesh: Mesh) -> SubMesh {
        SubMesh {
            object,
            group,
            material,
            material_data,
            mesh,
            transform: Transform::new(),
        }
    }

    pub fn draw(&self) {
        self.mesh.draw();
    }

    pub fn delete(&self) {
        self.mesh.delete();
    }
}