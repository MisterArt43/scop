use crate::texture::Texture;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct Mtl {
    pub name: String,
    pub ambient_color: [f32; 3],
    pub diffuse_color: [f32; 3],
    pub specular_color: [f32; 3],
    pub shininess: f32,
    pub diffuse_texture: Option<String>,
    pub diffuse_texture_data: Option<Texture>,
    pub index_of_refraction: Option<f32>,
    pub alpha: Option<f32>,
    pub specular_texture: Option<String>,
    pub specular_texture_data: Option<Texture>,
    pub illumination_model: Option<u32>,
}

impl Mtl {
    pub fn parse_mtl(path: &str, obj_path: &str) -> Result<HashMap<String, Mtl>, String> {
        let obj_dir = Path::new(obj_path).parent().unwrap_or(Path::new("./"));
        let mtl_path = obj_dir.join(path);
        let mtl_file = fs::read_to_string(&mtl_path)
            .map_err(|e| format!("Failed to read .mtl file: {}", e))?;

        let mut materials = HashMap::new();
        let mut current_material: Option<Mtl> = None;

        for line in mtl_file.lines() {
            let mut parts = line.split_whitespace();
            match parts.next() {
                Some("newmtl") => {
                    if let Some(mat) = current_material.take() {
                        materials.insert(mat.name.clone(), mat);
                    }
                    let name = line.split_once(' ').map(|(_, n)| n.trim().to_string()).unwrap_or_else(|| "default".to_string());
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
                    if let Some(mat) = &mut current_material {
                        mat.ambient_color = Self::parse_vec3(line.split_whitespace().skip(1));
                    }
                }
                Some("Kd") => {
                    if let Some(mat) = &mut current_material {
                        mat.diffuse_color = Self::parse_vec3(line.split_whitespace().skip(1));
                    }
                }
                Some("Ks") => {
                    if let Some(mat) = &mut current_material {
                        mat.specular_color = Self::parse_vec3(line.split_whitespace().skip(1));
                    }
                }
                Some("Ns") => {
                    if let Some(mat) = &mut current_material {
                        mat.shininess = line.split_whitespace().skip(1).next().and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0);
                    }
                }
                Some("map_Kd") => {
                    if let Some(mat) = &mut current_material {
                        let texture_path = Self::parse_mtl_texture_path(line);
                        let full_path = Self::resolve_texture_path(obj_dir, &texture_path);
                        let full_path_str = full_path.to_string_lossy().to_string();

                        println!("Chargement de la texture diffuse: {}", full_path_str);
                        mat.diffuse_texture_data = Texture::new(&full_path_str).ok();
                        mat.diffuse_texture = Some(full_path_str);
                    }
                }
                Some("Ni") => {
                    if let Some(mat) = &mut current_material {
                        mat.index_of_refraction = line.split_whitespace().skip(1).next().and_then(|s| s.parse::<f32>().ok());
                    }
                }
                Some("d") | Some("Tr") => {
                    if let Some(mat) = &mut current_material {
                        mat.alpha = line.split_whitespace().skip(1).next().and_then(|s| s.parse::<f32>().ok());
                    }
                }
                Some("map_Ks") | Some("map_Km") | Some("Km") => {
                    if let Some(mat) = &mut current_material {
                        let texture_path = Self::parse_mtl_texture_path(line);
                        let full_path = Self::resolve_texture_path(obj_dir, &texture_path);
                        let full_path_str = full_path.to_string_lossy().to_string();

                        println!("Chargement de la texture spéculaire: {}", full_path_str);
                        mat.specular_texture_data = Texture::new(&full_path_str).ok();
                        mat.specular_texture = Some(full_path_str);
                    }
                }
                Some("illum") => {
                    if let Some(mat) = &mut current_material {
                        mat.illumination_model = line.split_whitespace().skip(1).next().and_then(|s| s.parse::<u32>().ok());
                    }
                }
                _ => {}
            }
        }
        if let Some(mat) = current_material.take() {
            materials.insert(mat.name.clone(), mat);
        }
        Ok(materials)
    }

    fn parse_vec3(mut parts: std::iter::Skip<std::str::SplitWhitespace>) -> [f32; 3] {
        [
            parts.next().and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0),
            parts.next().and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0),
            parts.next().and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0),
        ]
    }

    fn parse_mtl_texture_path(line: &str) -> String {
        let tokens: Vec<_> = line.split_whitespace().collect();
        let mut index = 1;
        while index < tokens.len() {
            let token = tokens[index];
            if token.starts_with('-') {
                let extra = match token {
                    "-s" | "-o" | "-t" => 3,
                    "-mm" => 2,
                    _ => 1,
                };
                index += extra + 1;
                continue;
            }
            return tokens[index..].join(" ");
        }
        tokens.get(1).copied().unwrap_or("default").to_string()
    }

    fn resolve_texture_path(obj_dir: &Path, texture_path: &str) -> PathBuf {
        let candidate = obj_dir.join(texture_path.replace('\\', "/"));
        if candidate.exists() {
            return candidate;
        }

        let requested = Path::new(texture_path).file_stem().unwrap_or_default().to_string_lossy().to_ascii_lowercase();
        let requested_norm = Self::normalize_texture_name(&requested);

        let mut matches = Vec::new();
        Self::collect_texture_candidates(obj_dir, &mut matches);
        matches.retain(|path| path.extension().map(|ext| ext.eq_ignore_ascii_case("bmp") || ext.eq_ignore_ascii_case("ppm") || ext.eq_ignore_ascii_case("png") || ext.eq_ignore_ascii_case("jpg")).unwrap_or(false));
        matches.retain(|path| {
            let name = path.file_stem().unwrap_or_default().to_string_lossy().to_ascii_lowercase();
            let name_norm = Self::normalize_texture_name(&name);
            Self::texture_names_match(&requested_norm, &name_norm)
        });
        matches.sort();
        if let Some(path) = matches.into_iter().next() {
            return path;
        }
        candidate
    }

    fn collect_texture_candidates(dir: &Path, matches: &mut Vec<PathBuf>) {
        if let Ok(entries) = dir.read_dir() {
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();
                if path.is_dir() {
                    Self::collect_texture_candidates(&path, matches);
                } else if path.is_file() {
                    matches.push(path);
                }
            }
        }
    }

    fn normalize_texture_name(name: &str) -> String {
        name.replace('_', " ").replace('-', " ").replace('.', " ")
            .chars()
            .filter(|ch| ch.is_ascii_alphanumeric() || ch.is_ascii_whitespace())
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join("")
    }

    fn texture_names_match(requested: &str, candidate: &str) -> bool {
        if requested.is_empty() || candidate.is_empty() { return false; }
        if candidate.contains(requested) || requested.contains(candidate) { return true; }
        let req_wa = requested.replace("alpha", "").replace("apha", "");
        let cand_wa = candidate.replace("alpha", "").replace("apha", "");
        cand_wa.contains(&req_wa) || req_wa.contains(&cand_wa)
    }
}