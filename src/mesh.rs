use std::{collections::HashMap, fs::{self}, iter::Skip, mem::offset_of, ptr::null, str::SplitWhitespace, usize};
use gl::{DrawElements, FLOAT, TRIANGLES, UNSIGNED_INT};
use crate::{ebo::EBO, vao::VAO, vbo::VBO};

pub struct Mesh {
    pub vao: VAO,
    pub vbo: VBO,
    pub ebo: EBO,

    pub nb_vertices: usize,
    pub index_count: usize,
    pub bbox_min: [f32; 3],
    pub bbox_max: [f32; 3],
}

pub struct SubMesh {
    pub object: String,
    pub group: String,
    pub material: String,
    pub mesh: Mesh,
}

pub struct Vertex {
    pub(crate) position: [f32; 3],
    pub(crate) normal: [f32; 3],
    pub(crate) uv: [f32; 2],
    pub(crate) color: [f32; 3],
}

struct Builder {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    map: HashMap<VertexKey, u32>,
    object: String,
    group: String,
    material: String,
}

impl Builder {
    fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();
        self.map.clear();
        self.object = String::from("default");
        self.group = String::from("default");
        self.material = String::from("default");
    }

    fn is_empty(&self) -> bool {
        self.indices.is_empty()
    }
}

#[derive(Debug, Clone, Copy)]
struct ObjIndex {
    v: i32,              // index position
    vt: Option<i32>,     // index uv
    vn: Option<i32>,     // index normal
}
#[derive(PartialEq, Eq, Hash)]
struct VertexKey {
    v: usize,              // index position
    vt: Option<usize>,     // index uv
    vn: Option<usize>,     // index normal
}

impl Mesh {
    pub fn new(vertices: &Vec<Vertex>, indices: &Vec<u32>) -> Mesh {
        // compute bbox from vertex positions
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
            vbo: VBO::new(vertices,
                (vertices.len() * std::mem::size_of::<Vertex>()) as isize,
            ),
            ebo: EBO::new(
                indices,
                (indices.len() * std::mem::size_of::<u32>()) as isize,
            ),
            nb_vertices: vertices.len(),
            index_count: indices.len(),
            bbox_min,
            bbox_max,
        };
        mesh.vao.bind();
        mesh.ebo.bind();

        mesh.vao.link_attrib(&mesh.vbo, 0, 3, FLOAT, size_of::<Vertex>() as i32, offset_of!(Vertex, position));
        mesh.vao.link_attrib(&mesh.vbo, 1, 3, FLOAT, size_of::<Vertex>() as i32, offset_of!(Vertex, normal));
        mesh.vao.link_attrib(&mesh.vbo, 2, 2, FLOAT, size_of::<Vertex>() as i32, offset_of!(Vertex, uv));
        mesh.vao.link_attrib(&mesh.vbo, 3, 3, FLOAT, size_of::<Vertex>() as i32, offset_of!(Vertex, color));

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
        let obj_file: String = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read .obj file: {}", e))?;

        let mut builder = Builder {
            vertices: Vec::new(),
            indices: Vec::new(),
            map: HashMap::new(),
            object: String::from("default"),
            group: String::from("default"),
            material: String::from("default"),
        }; // le builder sera a flush par o/g/usemtl afin de gérer plusieurs mat/grp d'un même .obj

        let mut vertex_positions: Vec<[f32; 3]> = Vec::new();
        let mut vertex_normals: Vec<[f32; 3]> = Vec::new();
        let mut vertex_uvs: Vec<[f32; 2]> = Vec::new();

        let mut vertex_color = Vec::<Option<[f32; 3]>>::new();

        let mut meshes: Vec<SubMesh> = Vec::new(); // pour stocker les meshes d'un même .obj (o/g/usemtl)

        for line in  obj_file.lines() {
            match line.split_whitespace().next() {
                Some("v") => {
                    let mut parts = line.split_whitespace().skip(1);
                    let (position, color) = Self::parse_v(&mut parts);
                    vertex_positions.push(position);
                    vertex_color.push(color);
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
                    // Pour le parsing de f il y a plusieurs formats : 
                    // vIndex/vtIndex/vnIndex
                    // vIndex//vnIndex (pas d’uv)
                    // vIndex/vtIndex (pas de normal)
                    // vIndex (juste position)

                    //il faut pouvoir parser n-gones car f peut contenir de 3 a n vertices

                    let mut face_indices: Vec<u32> = Vec::new();
                    
                    let parts = line
                        .split_whitespace()
                        .skip(1); // découpe par vertice + skip le "f" du début
                    for part in parts {
                        // parsing de f pour choper les index et les résoudre car le format est 1-based (ca peut etre neg)
                        let key = Self::parse_face(part);
                        let key = VertexKey {
                            v: Self::resolve_vertex_index(key.v, vertex_positions.len()),
                            vt: key.vt.map(|i| Self::resolve_vertex_index(i, vertex_uvs.len())),
                            vn: key.vn.map(|i| Self::resolve_vertex_index(i, vertex_normals.len())),
                        };

                        // récupérer l'index si le vertex existe déjà ou sinon j'en créer un nouveau
                        let idx: u32 = if let Some(&existing_idx) = builder.map.get(&key) {
                            existing_idx
                        } else {
                            // recréer le vertex avec les index de f c/vt/vn
                            let mut color = vertex_color[key.v].unwrap_or([1.0; 3]);
                            // If color is black (0,0,0), use white instead as default
                            if color[0] == 0.0 && color[1] == 0.0 && color[2] == 0.0 {
                                color = [1.0; 3];
                            }
                            let vertex = Vertex {
                                position: vertex_positions[key.v],
                                normal: key.vn.map_or([0.0; 3], |vn_idx| vertex_normals[vn_idx]),
                                uv: key.vt.map_or([0.0; 2], |vt_idx| vertex_uvs[vt_idx]),
                                color,
                            };
                            // push le vertex dans le builder ! ET ! push l'index dans la map pour évité doublons
                            builder.vertices.push(vertex);
                            let idx = builder.vertices.len() as u32 - 1;
                            builder.map.insert(key, idx);

                            idx
                        };
                        // la face a ensuite un index pour chaque vertice de la face
                        face_indices.push(idx);
                    }

                    //une fois le for fini et qu'on a bien tout parsé, faut trianguler si la face a plus ou eg 3 vertices (ngone)
                    if face_indices.len() >= 3 {
                        let v0 = face_indices[0]; // tout les triangles d'un n-gone partent du premier vertex de la face
                        for i in 1..(face_indices.len() - 1) {
                            builder.indices.push(v0);
                            builder.indices.push(face_indices[i]);
                            builder.indices.push(face_indices[i + 1]);
                        }
                    }
                    else {
                        print!("Face with less than 3 vertices found in .obj file, skipping: {}", line);
                    }
                },
                Some("o") => {
                    Self::flush_builder_if_needed(&mut builder, &mut meshes); // flush le builder avant de commencer un nouveau mesh
                    builder.object = line.split_once(' ').map(|(_, name)| name.trim().to_string()).unwrap_or_else(|| "default".to_string());
                },
                Some("g") => {
                    Self::flush_builder_if_needed(&mut builder, &mut meshes); // flush le builder avant de commencer un nouveau mesh
                    builder.group = line.split_once(' ').map(|(_, name)| name.trim().to_string()).unwrap_or_else(|| "default".to_string());
                },
                Some("usemtl") => {
                    Self::flush_builder_if_needed(&mut builder, &mut meshes); // flush le builder avant de commencer un nouveau mesh
                    builder.material = line.split_once(' ').map(|(_, name)| name.trim().to_string()).unwrap_or_else(|| "default".to_string());
                },
                Some("#") => {
                    // pr éviter les print! dans les log
                },
                _ => {
                    print!("Unknown line in .obj file: {}", line);
                },
            }
        }

        Self::flush_builder_if_needed(&mut builder, &mut meshes); // Flush Final

        // TODO : parser le .obj et charger les vertices et indices dans les buffers
        Ok(meshes)
    }

    fn flush_builder_if_needed(builder: &mut Builder, meshes: &mut Vec<SubMesh>) {
        if !builder.is_empty() {
            let mesh = SubMesh {
                object: builder.object.clone(),
                group: builder.group.clone(),
                material: builder.material.clone(),
                mesh: Mesh::new(&builder.vertices, &builder.indices),
            };
            meshes.push(mesh);

            builder.clear();
        }
    }

    fn resolve_vertex_index(obj_index: i32, len: usize) -> usize {
        let len_i32 = len as i32;

        let obj_index = if obj_index < 0 {
            len_i32 + obj_index
        } else {
            obj_index - 1
        };

        if obj_index < 0 || obj_index >= len_i32 {
            panic!("Vertex index out of bounds: {}, len: {}", obj_index, len);
        }

        obj_index as usize
    }

    fn parse_face(token: &str) -> ObjIndex {        
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

        ObjIndex { v: v, vt: vt, vn: vn }
    }

    fn parse_v(token: &mut Skip<SplitWhitespace<'_>>) -> ([f32; 3], Option<[f32; 3]>) {
        let position = Self::parse_vec3(token);

        // Try to parse color from the next 3 values (if they exist)
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
