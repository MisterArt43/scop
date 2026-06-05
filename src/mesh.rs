use crate::{ebo::EBO, vao::VAO, vbo::VBO, texture::Texture};
use gl::{DrawElements, FLOAT, TRIANGLES, UNSIGNED_INT};
use std::{
    collections::HashMap,
    fs::{self},
    iter::Skip,
    mem::offset_of,
    ptr::null,
    str::SplitWhitespace,
    usize,
};

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
}

#[derive(Clone, Debug)]
pub struct Mtl {
    pub name: String,                     // name of the material
    pub ambient_color: [f32; 3],          // Ka
    pub diffuse_color: [f32; 3],          // Kd
    pub specular_color: [f32; 3],         // Ks
    pub shininess: f32,                   // Ns
    pub diffuse_texture: Option<String>,  // map_Kd (chemin du fichier)
    pub diffuse_texture_data: Option<Texture>, // Données de texture chargées
    pub index_of_refraction: Option<f32>, // Ni
    pub alpha: Option<f32>,               // d or Tr
    pub specular_texture: Option<String>, // map_Km (chemin du fichier)
    pub specular_texture_data: Option<Texture>, // Données de texture chargées
    pub illumination_model: Option<u32>,  // illum
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
    material_data: Option<Mtl>,
    face_count: u32,
}

impl Builder {
    fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();
        self.map.clear();
        self.object = String::from("default");
        self.group = String::from("default");
        self.material = String::from("default");
        self.material_data = None;
        self.face_count = 0;
    }

    fn is_empty(&self) -> bool {
        self.indices.is_empty()
    }
}

#[derive(Debug, Clone, Copy)]
struct ObjIndex {
    v: i32,          // index position
    vt: Option<i32>, // index uv
    vn: Option<i32>, // index normal
}
#[derive(PartialEq, Eq, Hash)]
struct VertexKey {
    v: usize,          // index position
    vt: Option<usize>, // index uv
    vn: Option<usize>, // index normal
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
            vbo: VBO::new(
                vertices,
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
        let obj_file: String =
            fs::read_to_string(path).map_err(|e| format!("Failed to read .obj file: {}", e))?;

        let mut mtl_data_map: HashMap<String, Mtl> = HashMap::new(); // pour stocker les données des matériaux parsés depuis les .mtl (clé : nom du matériau)
        let mut builder = Builder {
            vertices: Vec::new(),
            indices: Vec::new(),
            map: HashMap::new(),
            object: String::from("default"),
            group: String::from("default"),
            material: String::from("default"),
            material_data: None,
            face_count: 0,
        }; // le builder sera a flush par o/g/usemtl afin de gérer plusieurs mat/grp d'un même .obj

        let mut vertex_positions: Vec<[f32; 3]> = Vec::new();
        let mut vertex_normals: Vec<[f32; 3]> = Vec::new();
        let mut vertex_uvs: Vec<[f32; 2]> = Vec::new();

        let mut vertex_color = Vec::<Option<[f32; 3]>>::new();

        let mut meshes: Vec<SubMesh> = Vec::new(); // pour stocker les meshes d'un même .obj (o/g/usemtl)

        for line in obj_file.lines() {
            match line.split_whitespace().next() {
                Some("v") => {
                    let mut parts = line.split_whitespace().skip(1);
                    let (position, color) = Self::parse_v(&mut parts);
                    vertex_positions.push(position);
                    vertex_color.push(color);
                }
                Some("vn") => {
                    let mut parts = line.split_whitespace().skip(1);
                    vertex_normals.push(Self::parse_vec3(&mut parts));
                }
                Some("vt") => {
                    let mut parts = line.split_whitespace().skip(1);
                    vertex_uvs.push(Self::parse_vec2(&mut parts));
                }
                Some("f") => {
                    // Pour le parsing de f il y a plusieurs formats :
                    // vIndex/vtIndex/vnIndex
                    // vIndex//vnIndex (pas d’uv)
                    // vIndex/vtIndex (pas de normal)
                    // vIndex (juste position)

                    //il faut pouvoir parser n-gones car f peut contenir de 3 a n vertices

                    let mut face_indices: Vec<u32> = Vec::new();                    let current_face_index = builder.face_count;
                    builder.face_count += 1;
                    let parts = line.split_whitespace().skip(1); // découpe par vertice + skip le "f" du début
                    for part in parts {
                        // parsing de f pour choper les index et les résoudre car le format est 1-based (ca peut etre neg)
                        let key = Self::parse_face(part);
                        let key = VertexKey {
                            v: Self::resolve_vertex_index(key.v, vertex_positions.len()),
                            vt: key
                                .vt
                                .map(|i| Self::resolve_vertex_index(i, vertex_uvs.len())),
                            vn: key
                                .vn
                                .map(|i| Self::resolve_vertex_index(i, vertex_normals.len())),
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
                                uv: key.vt.map_or_else(
                                    || Self::generate_uv_from_position(&vertex_positions[key.v], current_face_index),
                                    |vt_idx| vertex_uvs[vt_idx]
                                ),
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
                    } else {
                        println!(
                            "Face with less than 3 vertices found in .obj file, skipping: {}",
                            line
                        );
                    }
                }
                Some("o") => {
                    Self::flush_builder_if_needed(&mut builder, &mut meshes); // flush le builder avant de commencer un nouveau mesh
                    builder.object = line
                        .split_once(' ')
                        .map(|(_, name)| name.trim().to_string())
                        .unwrap_or_else(|| "default".to_string());
                }
                Some("g") => {
                    Self::flush_builder_if_needed(&mut builder, &mut meshes); // flush le builder avant de commencer un nouveau mesh
                    builder.group = line
                        .split_once(' ')
                        .map(|(_, name)| name.trim().to_string())
                        .unwrap_or_else(|| "default".to_string());
                }
                Some("mtllib") => {
                    let mtl_path = line
                        .split_once(' ')
                        .map(|(_, path)| path.trim())
                        .unwrap_or("default.mtl");
                    println!("parsing mtl file at path : {}", mtl_path);

                    if let Ok(materials) = Self::parse_mtl(mtl_path, path) {
                        for (name, mtl) in materials {
                            mtl_data_map.insert(name, mtl);
                        }
                    }
                }
                Some("usemtl") => {
                    Self::flush_builder_if_needed(&mut builder, &mut meshes); // flush le builder avant de commencer un nouveau mesh

                    builder.material = line
                        .split_once(' ')
                        .map(|(_, name)| name.trim().to_string())
                        .unwrap_or_else(|| "default".to_string());
                    println!("parse material : {}", builder.material);
                    builder.material_data =
                        Self::match_mtl(&builder.material, &mtl_data_map).cloned();

                    // println!("Material data for {}: {:?}", builder.material, builder.material_data);
                }
                Some("#") => {
                    // pr éviter les print! dans les log
                }
                _ => {
                    if !line.trim().is_empty() {
                        println!("Unknown line in .obj file: {}", line);
                    }
                }
            }
        }

        Self::flush_builder_if_needed(&mut builder, &mut meshes); // Flush Final

        // TODO : parser le .obj et charger les vertices et indices dans les buffers
        Ok(meshes)
    }

    fn match_mtl<'a>(name: &str, mtl_data_map: &'a HashMap<String, Mtl>) -> Option<&'a Mtl> {
        mtl_data_map.get(name)
    }

    fn parse_mtl(path: &str, obj_path: &str) -> Result<HashMap<String, Mtl>, String> {
        let obj_dir = std::path::Path::new(obj_path)
            .parent()
            .unwrap_or(std::path::Path::new("./"));
        let mtl_path = obj_dir.join(path);
        let mtl_file: String = fs::read_to_string(&mtl_path)
            .map_err(|e| format!("Failed to read .mtl file: {}", e))?;

        let mut materials: HashMap<String, Mtl> = HashMap::new();
        let mut current_material: Option<Mtl> = None;
        mtl_file.lines().for_each(|line| {
            match line.split_whitespace().next() {
                Some("newmtl") => {
                    if let Some(mat) = current_material.take() {
                        materials.insert(mat.name.clone(), mat);
                    }
                    let name = line
                        .split_once(' ')
                        .map(|(_, name)| name.trim().to_string())
                        .unwrap_or_else(|| "default".to_string());
                    current_material = Some(Mtl {
                        name,
                        ambient_color: [0.0; 3],
                        diffuse_color: [0.0; 3],
                        specular_color: [0.0; 3],
                        shininess: 0.0,
                        diffuse_texture: None,
                        diffuse_texture_data: None,
                        index_of_refraction: None,
                        alpha: None,
                        specular_texture: None,
                        specular_texture_data: None,
                        illumination_model: None,
                    });
                }
                Some("Ka") => {
                    // ambient color
                    if let Some(mat) = &mut current_material {
                        let mut parts = line.split_whitespace().skip(1);
                        mat.ambient_color = Self::parse_vec3(&mut parts);
                    }
                }
                Some("Kd") => {
                    // diffuse color
                    if let Some(mat) = &mut current_material {
                        let mut parts = line.split_whitespace().skip(1);
                        mat.diffuse_color = Self::parse_vec3(&mut parts);
                    }
                }
                Some("Ks") => {
                    // specular color
                    if let Some(mat) = &mut current_material {
                        let mut parts = line.split_whitespace().skip(1);
                        mat.specular_color = Self::parse_vec3(&mut parts);
                    }
                }
                Some("Ns") => {
                    // shininess
                    if let Some(mat) = &mut current_material {
                        let mut parts = line.split_whitespace().skip(1);
                        mat.shininess = parts
                            .next()
                            .and_then(|s| s.parse::<f32>().ok())
                            .unwrap_or(0.0);
                    }
                }
                Some("map_Kd") => {
                    // diffuse texture
                    if let Some(mat) = &mut current_material {
                        let texture_path = line
                            .split_once(' ')
                            .map(|(_, path)| path.trim().to_string())
                            .unwrap_or_else(|| "default".to_string());
                        
                        // Construire le chemin complet
                        let full_path = obj_dir.join(&texture_path);
                        let full_path_str = full_path.to_string_lossy().to_string();
                        
                        // Charger la texture
                        println!("Chargement de la texture diffuse: {}", full_path_str);
                        match Texture::new(&full_path_str) {
                            Ok(texture_data) => {
                                mat.diffuse_texture_data = Some(texture_data);
                                println!("✓ Texture diffuse chargée avec succès");
                            }
                            Err(e) => {
                                eprintln!("✗ Erreur lors du chargement de la texture diffuse: {}", e);
                            }
                        }
                        
                        mat.diffuse_texture = Some(full_path_str);
                    }
                }
                Some("Ni") => {
                    // index of refraction
                    if let Some(mat) = &mut current_material {
                        let mut parts = line.split_whitespace().skip(1);
                        mat.index_of_refraction = parts.next().and_then(|s| s.parse::<f32>().ok());
                    }
                }
                Some("d") => {
                    // alpha (transparency)
                    if let Some(mat) = &mut current_material {
                        let mut parts = line.split_whitespace().skip(1);
                        mat.alpha = parts.next().and_then(|s| s.parse::<f32>().ok());
                    }
                }
                Some("Tr") => {
                    // transparency
                    if let Some(mat) = &mut current_material {
                        let mut parts = line.split_whitespace().skip(1);
                        mat.alpha = parts.next().and_then(|s| s.parse::<f32>().ok());
                    }
                }
                Some("map_Ks") | Some("map_Km") | Some("Km") => {
                    // specular texture
                    if let Some(mat) = &mut current_material {
                        let texture_path = line
                            .split_once(' ')
                            .map(|(_, path)| path.trim().to_string())
                            .unwrap_or_else(|| "default".to_string());
                        
                        // Construire le chemin complet
                        let full_path = obj_dir.join(&texture_path);
                        let full_path_str = full_path.to_string_lossy().to_string();
                        
                        // Charger la texture
                        println!("Chargement de la texture spéculaire: {}", full_path_str);
                        match Texture::new(&full_path_str) {
                            Ok(texture_data) => {
                                mat.specular_texture_data = Some(texture_data);
                                println!("✓ Texture spéculaire chargée avec succès");
                            }
                            Err(e) => {
                                eprintln!("✗ Erreur lors du chargement de la texture spéculaire: {}", e);
                            }
                        }
                        
                        mat.specular_texture = Some(full_path_str);
                    }
                }
                Some("illum") => {
                    // illumination model
                    if let Some(mat) = &mut current_material {
                        let mut parts = line.split_whitespace().skip(1);
                        let illum_model = parts
                            .next()
                            .and_then(|s| s.parse::<u32>().ok())
                            .unwrap_or(0);
                        mat.illumination_model = Some(illum_model);
                    }
                }
                Some("#") => {
                    // comment, ignore
                }
                _ => {
                    if !line.trim().is_empty() {
                        println!("Unknown line in .mtl file: {}", line);
                    }
                }
            }
        });
        if let Some(mat) = current_material.take() {
            materials.insert(mat.name.clone(), mat);
        }
        println!("Parsed {} materials from .mtl file", materials.len());
        Ok(materials)
    }

    fn flush_builder_if_needed(builder: &mut Builder, meshes: &mut Vec<SubMesh>) {
        if !builder.is_empty() {
            let mesh = SubMesh {
                object: builder.object.clone(),
                group: builder.group.clone(),
                material: builder.material.clone(),
                mesh: Mesh::new(&builder.vertices, &builder.indices),
                material_data: builder.material_data.clone(),
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

        ObjIndex {
            v: v,
            vt: vt,
            vn: vn,
        }
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

    fn parse_vec2(parts: &mut Skip<SplitWhitespace<'_>>) -> [f32; 2] {
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

    pub fn delete(&self) {
        self.vao.delete();
        self.vbo.delete();
        self.ebo.delete();
    }
}
