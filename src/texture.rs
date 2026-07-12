use crate::bmp;

#[derive(Clone, Debug)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[derive(Clone, Debug)]
pub struct Texture {
    pub width: u32,
    pub height: u32,
    pixels: Vec<Pixel>,
    pub texture_id: Option<u32>, // OpenGL texture ID
}

/**
 * This implementation will parse different image formats
 * and create a texture object.
 */
impl Texture {
    pub fn new(path: &str) -> Result<Texture, String> {
        let mut texture = Texture {
            width: 0,
            height: 0,
            pixels: Vec::new(),
            texture_id: None,
        };

        match find_image_format(path) {
            Some("bmp") => {
                // Parse BMP en appelant le parser depuis bmp.rs
                match bmp::load_bmp_as_texture(path) {
                    Ok(texture_data) => {
                        texture.width = texture_data.width;
                        texture.height = texture_data.height;
                        // Convertir les données RGBA en vec de Pixel
                        for chunk in texture_data.pixels.chunks(4) {
                            if chunk.len() == 4 {
                                texture.pixels.push(Pixel {
                                    r: chunk[0],
                                    g: chunk[1],
                                    b: chunk[2],
                                    a: chunk[3],
                                });
                            }
                        }
                        println!(
                            " - BMP chargé avec succès: {}x{}",
                            texture.width, texture.height
                        );
                    }
                    Err(e) => {
                        return Err(format!("Erreur lors du chargement du BMP {}: {}", path, e));
                    }
                }
            }
            _ => return Err(format!("Format non supporté pour le fichier: {}", path)),
        }

        Ok(texture)
    }

    /// Charge la texture en OpenGL et retourne le texture ID
    pub fn load_to_gpu(&mut self) -> Result<u32, String> {
        if self.width == 0 || self.height == 0 {
            return Err("Texture a une taille invalide".to_string());
        }

        // Convertir les pixels en array RGBA
        let mut rgba_data = Vec::with_capacity((self.width * self.height * 4) as usize);
        for pixel in &self.pixels {
            rgba_data.push(pixel.r);
            rgba_data.push(pixel.g);
            rgba_data.push(pixel.b);
            rgba_data.push(pixel.a);
        }

        let mut texture_id: u32 = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            // Configuration des paramètres de texture
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            // Uploader les données à la GPU
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                self.width as i32,
                self.height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                rgba_data.as_ptr() as *const _,
            );

            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        self.texture_id = Some(texture_id);
        println!(" - Texture GPU chargée avec l'ID: {}", texture_id);

        Ok(texture_id)
    }

    /// Binder la texture pour le rendu
    pub fn bind(&self, slot: u32) {
        if let Some(id) = self.texture_id {
            unsafe {
                gl::ActiveTexture(gl::TEXTURE0 + slot);
                gl::BindTexture(gl::TEXTURE_2D, id);
            }
        }
    }

    /// Unbinder la texture
    pub fn unbind() {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    /// Supprimer la texture de la GPU
    pub fn delete(&self) {
        if let Some(id) = self.texture_id {
            unsafe {
                gl::DeleteTextures(1, &id);
            }
        }
    }
}

// Helper function to determine image format
// will only handle simple formats
// because i habe to code the parsing myself
fn find_image_format(path: &str) -> Option<&str> {
    if path.ends_with(".ppm") {
        Some("ppm")
    } else if path.ends_with(".bmp") {
        Some("bmp")
    } else {
        None
    }
}
