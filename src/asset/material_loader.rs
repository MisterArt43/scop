use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    rc::Rc,
};

use anyhow::Result;

use crate::{
    asset::{missing_texture_path, mtl::MtlMaterial}, gfx::{
        material::Material,
        texture::{Texture, TextureFilter, TextureWrap},
    }, image::Image,
};

pub struct MaterialLoader {
    base_dir: PathBuf,
    textures: HashMap<PathBuf, Rc<Texture>>,
}

impl MaterialLoader {
    pub fn new(base_dir: impl Into<PathBuf>) -> Self {
        Self {
            base_dir: base_dir.into(),
            textures: HashMap::new(),
        }
    }

    pub fn load(&mut self, mtl: &MtlMaterial) -> Result<Material> {
        let diffuse_texture = match &mtl.diffuse_texture {
            Some(path) => Some(self.load_texture(path)?),
            None => None,
        };

        let specular_texture = match &mtl.specular_texture {
            Some(path) => Some(self.load_texture(path)?),
            None => None,
        };

        Ok(Material::from_data(
            mtl.name.clone(),
            mtl.ambient,
            mtl.diffuse,
            mtl.specular,
            mtl.shininess,
            mtl.opacity,
            diffuse_texture,
            specular_texture,
        ))
    }

    fn load_texture(&mut self, path: &Path) -> Result<Rc<Texture>> {
        let path = self.resolve_texture_path(path);

        if let Some(texture) = self.textures.get(&path) {
            return Ok(texture.clone());
        }

        let mut image = Image::load_from_file_or_default(&path, &missing_texture_path());
        image.flipv();

        let mut texture = Texture::new_2d();

        texture.set_wrap(TextureWrap::Repeat, TextureWrap::Repeat);
    	texture.set_filter(
                TextureFilter::LinearMipmapLinear,
                TextureFilter::Linear,
            );
        texture.upload(
                image.width,
                image.height,
                &image.pixels,
                image.get_texture_format(),
            )
            .generate_mipmaps();

        let texture = Rc::new(texture);

        self.textures.insert(path, texture.clone());

        Ok(texture)
    }

    fn resolve_texture_path(&self, path: &Path) -> PathBuf {
        let path = self.base_dir.join(path);

        if path.exists() {
            return path;
        }

        let name = path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_ascii_lowercase();

        find_texture(&self.base_dir, &name).unwrap_or(path)
    }
}

fn find_texture(dir: &Path, name: &str) -> Option<PathBuf> {
    for entry in dir.read_dir().ok()?.flatten() {
        let path = entry.path();

        if path.is_dir() {
            if let Some(found) = find_texture(&path, name) {
                return Some(found);
            }

            continue;
        }

        let ext = path
            .extension()?
            .to_string_lossy()
            .to_ascii_lowercase();

        if !matches!(ext.as_str(), "bmp" | "ppm") {
            continue;
        }

        let file_name = path
            .file_stem()?
            .to_string_lossy()
            .to_ascii_lowercase();

        if file_name.contains(name) || name.contains(&file_name) {
            return Some(path);
        }
    }

    None
}