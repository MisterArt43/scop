use std::{fs::{self}, iter::Skip, mem::{offset_of}, ptr::null, str::SplitWhitespace};

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

    pub fn load_from_obj(path: &str) -> Mesh {
        let obj_file: String = fs::read_to_string(path)
        .expect("Failed to read .obj file");

        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        let mut vertex_positions: Vec<[f32; 3]> = Vec::new();
        let mut vertex_normals: Vec<[f32; 3]> = Vec::new();
        let mut vertex_uvs: Vec<[f32; 2]> = Vec::new();

        for line in  obj_file.lines() {
            match line.split_whitespace().next() {
                Some("v") => {
                    let mut parts = line.split_whitespace().skip(1);
                    vertex_positions.push(Self::parse_vec3(&mut parts));
                },
                Some("vn") => {
                    let mut parts = line.split_whitespace().skip(1);
                    vertex_normals.push(Self::parse_vec3(&mut parts));
                },
                Some("vt") => {
                    let mut parts = line.split_whitespace().skip(1);
                    vertex_uvs.push(Self::parse_vec2(&mut parts));
                },
                Some("f") => {
                    // TODO/notabene pour plus tard pcq flemme
                    // Pour le parsing de f il y a plusieurs formats : 
                    // vIndex/vtIndex/vnIndex
                    // ou vIndex//vnIndex (pas d’uv)
                    // ou vIndex/vtIndex (pas de normal)
                    // ou vIndex (juste position)

                    //il faut pouvoir parser n-gones car f peut contenir de 3 a n vertices
                    
                    let mut parts = line
                        .split_whitespace()
                        .skip(1); // on découpe par vertice + skip le "f" du début
                    for mut part in parts {
                        let (v, vt, vn) = Self::parse_face(part);
                    }
                    // face (indices)
                },
                _ => {},
            }
        }

        // TODO : parser le .obj et charger les vertices et indices dans les buffers
        Mesh::new(&Vec::new(), &Vec::new())
    }

    fn parse_face(token: &str) -> (i32, Option<i32>, Option<i32>) {

        
        let mut it = token.split('/');
        let v = it
            .next()
            .and_then(|s| s.parse::<i32>().ok())
            .expect("Failed to parse vertex index");

        let vt = it
            .next()
            .and_then(|s| {
                if s.is_empty() { None } 
                else { s.parse::<i32>().ok() }
            });
        
        let vn = it
            .next()
            .and_then(|s| {
                if s.is_empty() { None }
                else { s.parse::<i32>().ok() }
            });

        (v, vt, vn)
    }

    fn parse_vec3(parts: &mut Skip<SplitWhitespace<'_>>) -> [f32; 3] {
        [
            parts.next()
                .and_then(|s| s.parse::<f32>().ok())
                .unwrap_or(0.0),
            parts.next()
                .and_then(|s| s.parse::<f32>().ok())
                .unwrap_or(0.0),
            parts.next()
                .and_then(|s| s.parse::<f32>().ok())
                .unwrap_or(0.0),
        ]
    }

    fn parse_vec2(parts: &mut Skip<SplitWhitespace<'_>>) -> [f32; 2] {
        [
            parts.next()
                .and_then(|s| s.parse::<f32>().ok())
                .unwrap_or(0.0),
            parts.next()
                .and_then(|s| s.parse::<f32>().ok())
                .unwrap_or(0.0),
        ]
    }

    pub fn delete(&self) {
        self.vao.delete();
        self.vbo.delete();
        self.ebo.delete();
    }
}
