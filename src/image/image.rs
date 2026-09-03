use std::path::Path;

use crate::gfx::texture;

#[derive(Debug, Clone)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub channels: u8,
    pub pixels: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ImageExtension {
    BMP,
    PPM,
}

impl Image {
    pub fn new(width: u32, height: u32, channels: u8, pixels: Vec<u8>) -> Self {
        Self {
            width,
            height,
            channels,
            pixels,
        }
    }

    pub fn flipv(&mut self) -> &mut Self {
        let row_size = (self.width * self.channels as u32) as usize;
        let half_height = self.height / 2;

        for y in 0..half_height {
            let top_index = y as usize * row_size;
            let bottom_index = (self.height - 1 - y) as usize * row_size;

            let (before_bottom, bottom) = self.pixels.split_at_mut(bottom_index);
            before_bottom[top_index..top_index + row_size].swap_with_slice(&mut bottom[..row_size]);
        }

        self
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.pixels
    }

    pub fn pixels(&self) -> &[u8] {
        &self.pixels
    }

    pub fn get_texture_format(&self) -> texture::TextureFormat {
        match self.channels {
            1 => texture::TextureFormat::R8,
            2 => texture::TextureFormat::RG8,
            3 => texture::TextureFormat::RGB8,
            4 => texture::TextureFormat::RGBA8,
            _ => panic!("Unsupported number of channels: {}", self.channels),
        }
    }

    pub fn load_from_file(path: &Path) -> Option<Self> {
    match path
        .extension()?
        .to_str()?
        .to_ascii_lowercase()
        .as_str()
    {
        "bmp" => crate::image::bmp::BMP::load(path).ok(),
        "ppm" => crate::image::ppm::PPM::load(path).ok(),
        _ => None,
    }
}

    pub fn load_from_file_or_default(path: &Path, default_path: &Path) -> Self {
        match Self::load_from_file(path) {
            Some(image) => image,
            None => {
                eprintln!(
                    "Failed to load image from '{}', loading default image from '{}'",
                    path.display(), default_path.display()
                );
                Self::load_from_file(default_path).expect("Failed to load default image")
            }
        }
    }

    pub fn default_missing_texture_path() -> &'static str {
        "assets/textures/missing_texture.bmp"
    }
}
