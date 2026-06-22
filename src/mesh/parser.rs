use crate::material::Mtl;
use crate::mesh::{
    Mesh, SubMesh, Vertex,
    vertex::{ObjIndex, VertexKey},
};
use std::collections::HashMap;
use std::fs;

pub struct Builder {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub map: HashMap<VertexKey, u32>,
    pub object: String,
    pub group: String,
    pub material: String,
    pub material_data: Option<Mtl>,
    pub face_count: u32,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            map: HashMap::new(),
            object: String::from("default"),
            group: String::from("default"),
            material: String::from("default"),
            material_data: None,
            face_count: 0,
        }
    }

    pub fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();
        self.map.clear();
        self.object = String::from("default");
        self.group = String::from("default");
        self.material = String::from("default");
        self.material_data = None;
        self.face_count = 0;
    }

    pub fn is_empty(&self) -> bool {
        self.indices.is_empty()
    }
}

pub fn parse_obj(path: &str) -> Result<Vec<SubMesh>, String> {
    let obj_file =
        fs::read_to_string(path).map_err(|e| format!("Failed to read .obj file: {}", e))?;
    let mut mtl_data_map: HashMap<String, Mtl> = HashMap::new();
    let mut builder = Builder::new();

    let mut vertex_positions = Vec::new();
    let mut vertex_normals = Vec::new();
    let mut vertex_uvs = Vec::new();
    let mut vertex_colors = Vec::new();
    let mut meshes = Vec::new();

    for line in obj_file.lines() {
        match line.split_whitespace().next() {
            Some("v") => {
                let (pos, col) = parse_v(line.split_whitespace().skip(1));
                vertex_positions.push(pos);
                vertex_colors.push(col);
            }
            Some("vn") => vertex_normals.push(parse_vec3(line.split_whitespace().skip(1))),
            Some("vt") => vertex_uvs.push(parse_vec2(line.split_whitespace().skip(1))),
            Some("f") => {
                let mut face_indices = Vec::new();
                let current_face_index = builder.face_count;
                builder.face_count += 1;

                for part in line.split_whitespace().skip(1) {
                    let raw_key = parse_face(part);
                    let key = VertexKey {
                        v: resolve_vertex_index(raw_key.v, vertex_positions.len()),
                        vt: raw_key
                            .vt
                            .map(|i| resolve_vertex_index(i, vertex_uvs.len())),
                        vn: raw_key
                            .vn
                            .map(|i| resolve_vertex_index(i, vertex_normals.len())),
                    };

                    let idx = if let Some(&existing_idx) = builder.map.get(&key) {
                        existing_idx
                    } else {
                        let mut color = vertex_colors[key.v].unwrap_or([1.0; 3]);
                        if color == [0.0, 0.0, 0.0] {
                            color = [1.0; 3];
                        }

                        let vertex = Vertex {
                            position: vertex_positions[key.v],
                            normal: key.vn.map_or([0.0; 3], |vn_idx| vertex_normals[vn_idx]),
                            uv: key.vt.map_or_else(
                                || {
                                    generate_uv_from_position(
                                        &vertex_positions[key.v],
                                        current_face_index,
                                    )
                                },
                                |vt_idx| vertex_uvs[vt_idx],
                            ),
                            color,
                        };
                        builder.vertices.push(vertex);
                        let new_idx = builder.vertices.len() as u32 - 1;
                        builder.map.insert(key, new_idx);
                        new_idx
                    };
                    face_indices.push(idx);
                }

                if face_indices.len() >= 3 {
                    let v0 = face_indices[0];
                    for i in 1..(face_indices.len() - 1) {
                        builder.indices.push(v0);
                        builder.indices.push(face_indices[i]);
                        builder.indices.push(face_indices[i + 1]);
                    }
                }
            }
            Some("o") => {
                flush_builder_if_needed(&mut builder, &mut meshes);
                builder.object = line
                    .split_once(' ')
                    .map(|(_, n)| n.trim().to_string())
                    .unwrap_or_else(|| "default".to_string());
            }
            Some("g") => {
                flush_builder_if_needed(&mut builder, &mut meshes);
                builder.group = line
                    .split_once(' ')
                    .map(|(_, n)| n.trim().to_string())
                    .unwrap_or_else(|| "default".to_string());
            }
            Some("mtllib") => {
                let mtl_path = line
                    .split_once(' ')
                    .map(|(_, p)| p.trim())
                    .unwrap_or("default.mtl");
                if let Ok(materials) = Mtl::parse_mtl(mtl_path, path) {
                    mtl_data_map.extend(materials);
                }
            }
            Some("usemtl") => {
                flush_builder_if_needed(&mut builder, &mut meshes);
                builder.material = line
                    .split_once(' ')
                    .map(|(_, n)| n.trim().to_string())
                    .unwrap_or_else(|| "default".to_string());
                builder.material_data = mtl_data_map.get(&builder.material).cloned();
            }
            _ => {}
        }
    }
    flush_builder_if_needed(&mut builder, &mut meshes);
    Ok(meshes)
}

fn flush_builder_if_needed(builder: &mut Builder, meshes: &mut Vec<SubMesh>) {
    if !builder.is_empty() {
        meshes.push(SubMesh {
            object: builder.object.clone(),
            group: builder.group.clone(),
            material: builder.material.clone(),
            mesh: Mesh::new(&builder.vertices, &builder.indices),
            material_data: builder.material_data.clone(),
        });
        builder.clear();
    }
}

fn resolve_vertex_index(obj_index: i32, len: usize) -> usize {
    let len_i32 = len as i32;
    let resolved = if obj_index < 0 {
        len_i32 + obj_index
    } else {
        obj_index - 1
    };
    if resolved < 0 || resolved >= len_i32 {
        panic!("Vertex index out of bounds: {}, len: {}", resolved, len);
    }
    resolved as usize
}

fn parse_face(token: &str) -> ObjIndex {
    let mut it = token.split('/');
    let v = it
        .next()
        .and_then(|s| s.parse::<i32>().ok())
        .expect("Failed to parse vertex index");
    let vt = it.next().and_then(|s| {
        if s.is_empty() {
            None
        } else {
            s.parse::<i32>().ok()
        }
    });
    let vn = it.next().and_then(|s| {
        if s.is_empty() {
            None
        } else {
            s.parse::<i32>().ok()
        }
    });
    ObjIndex { v, vt, vn }
}

fn parse_v(token: std::iter::Skip<std::str::SplitWhitespace<'_>>) -> ([f32; 3], Option<[f32; 3]>) {
    let position = parse_vec3(token.clone());
    let mut token = token.skip(3);
    let color = [
        token.next().and_then(|s| s.parse::<f32>().ok()),
        token.next().and_then(|s| s.parse::<f32>().ok()),
        token.next().and_then(|s| s.parse::<f32>().ok()),
    ];
    let color = if color.iter().all(|c| c.is_some()) {
        Some([color[0].unwrap(), color[1].unwrap(), color[2].unwrap()])
    } else {
        None
    };
    (position, color)
}

fn parse_vec3(mut parts: std::iter::Skip<std::str::SplitWhitespace<'_>>) -> [f32; 3] {
    [
        parts
            .next()
            .and_then(|s| s.parse::<f32>().ok())
            .unwrap_or(0.0),
        parts
            .next()
            .and_then(|s| s.parse::<f32>().ok())
            .unwrap_or(0.0),
        parts
            .next()
            .and_then(|s| s.parse::<f32>().ok())
            .unwrap_or(0.0),
    ]
}

fn parse_vec2(mut parts: std::iter::Skip<std::str::SplitWhitespace<'_>>) -> [f32; 2] {
    [
        parts
            .next()
            .and_then(|s| s.parse::<f32>().ok())
            .unwrap_or(0.0),
        parts
            .next()
            .and_then(|s| s.parse::<f32>().ok())
            .unwrap_or(0.0),
    ]
}

fn generate_uv_from_position(pos: &[f32; 3], face_index: u32) -> [f32; 2] {
    let hue = (face_index % 10) as f32 / 10.0;
    [hue, (pos[1] + 1.0) / 2.0]
}
