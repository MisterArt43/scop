use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::math::vec3::Vec3;

/*==============================================================*/
/*                        MtlMaterial                           */
/*==============================================================*/

#[derive(Debug, Clone)]
pub struct MtlMaterial {
    pub name: String,

    pub ambient: Vec3,
    pub diffuse: Vec3,
    pub specular: Vec3,

    pub shininess: f32,
    pub opacity: f32,

    pub index_of_refraction: Option<f32>,
    pub illumination_model: Option<u32>,

    pub diffuse_texture: Option<PathBuf>,
    pub specular_texture: Option<PathBuf>,
}

impl MtlMaterial {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),

            ambient: Vec3::ZERO,
            diffuse: Vec3::ONE,
            specular: Vec3::ZERO,

            shininess: 1.0,
            opacity: 1.0,

            index_of_refraction: None,
            illumination_model: None,

            diffuse_texture: None,
            specular_texture: None,
        }
    }
}

/*==============================================================*/
/*                             Mtl                              */
/*==============================================================*/

#[derive(Debug, Clone)]
pub struct Mtl;

impl Mtl {
    pub fn load(path: &str, obj_path: &str) -> Result<HashMap<String, MtlMaterial>, String> {
        let obj_dir = Path::new(obj_path).parent().unwrap_or(Path::new("."));

        let path = obj_dir.join(path);

        let file = std::fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read '{}': {e}", path.display()))?;

        let mut materials = HashMap::new();
        let mut current: Option<MtlMaterial> = None;

        for line in file.lines() {
            let line = line.trim();

            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let mut parts = line.split_whitespace();

            match parts.next() {
                Some("newmtl") => {
                    if let Some(material) = current.take() {
                        materials.insert(material.name.clone(), material);
                    }

                    current = Some(MtlMaterial::new(parts.next().unwrap_or("default")));
                }

                Some("Ka") => {
                    if let Some(material) = &mut current {
                        material.ambient = Self::parse_vec3(parts);
                    }
                }

                Some("Kd") => {
                    if let Some(material) = &mut current {
                        material.diffuse = Self::parse_vec3(parts);
                    }
                }

                Some("Ks") => {
                    if let Some(material) = &mut current {
                        material.specular = Self::parse_vec3(parts);
                    }
                }

                Some("Ns") => {
                    if let Some(material) = &mut current {
                        material.shininess = Self::parse_f32(parts);
                    }
                }

                Some("Ni") => {
                    if let Some(material) = &mut current {
                        material.index_of_refraction = parts.next().and_then(|v| v.parse().ok());
                    }
                }

                Some("d") => {
                    if let Some(material) = &mut current {
                        material.opacity = Self::parse_f32(parts);
                    }
                }

                Some("Tr") => {
                    if let Some(material) = &mut current {
                        material.opacity = 1.0 - Self::parse_f32(parts);
                    }
                }

                Some("illum") => {
                    if let Some(material) = &mut current {
                        material.illumination_model = parts.next().and_then(|v| v.parse().ok());
                    }
                }

                Some("map_Kd") => {
                    if let Some(material) = &mut current {
                        material.diffuse_texture = Self::parse_texture_path(line);
                    }
                }

                Some("map_Ks") | Some("map_Km") | Some("Km") => {
                    if let Some(material) = &mut current {
                        material.specular_texture = Self::parse_texture_path(line);
                    }
                }

                _ => {}
            }
        }

        if let Some(material) = current {
            materials.insert(material.name.clone(), material);
        }

        Ok(materials)
    }

    fn parse_vec3<'a>(mut parts: impl Iterator<Item = &'a str>) -> Vec3 {
        Vec3::new(
            parts.next().and_then(|v| v.parse().ok()).unwrap_or(0.0),
            parts.next().and_then(|v| v.parse().ok()).unwrap_or(0.0),
            parts.next().and_then(|v| v.parse().ok()).unwrap_or(0.0),
        )
    }

    fn parse_f32<'a>(mut parts: impl Iterator<Item = &'a str>) -> f32 {
        parts.next().and_then(|v| v.parse().ok()).unwrap_or(0.0)
    }

    fn parse_texture_path(line: &str) -> Option<PathBuf> {
        let tokens: Vec<_> = line.split_whitespace().collect();
        let mut i = 1;

        while i < tokens.len() {
            if !tokens[i].starts_with('-') {
                return Some(PathBuf::from(tokens[i..].join(" ")));
            }

            i += match tokens[i] {
                "-s" | "-o" | "-t" => 4,
                "-mm" => 3,
                _ => 2,
            };
        }

        None
    }
}
