use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

// ============================================================================
// ÉTAPE 1 : Structures pour les headers BMP
// ============================================================================

/// File Header du BMP (14 bytes)
/// Contient les informations générales du fichier
#[derive(Debug, Clone)]
pub struct BmpFileHeader {
    pub file_size: u32,      // Taille totale du fichier en bytes
    pub pixel_offset: u32,   // Offset où commencent les pixel data
}

/// DIB Header du BMP (40 bytes - info header standard)
/// Contient les informations sur l'image
#[derive(Debug, Clone)]
pub struct BmpDibHeader {
    pub width: i32,          // Largeur en pixels
    pub height: i32,         // Hauteur en pixels
    pub planes: u16,         // Doit toujours être 1
    pub bits_per_pixel: u16, // 1, 4, 8, 16, 24, 32
    pub compression: u32,    // 0 = pas de compression (ce qu'on veut)
    pub image_size: u32,     // Taille des pixel data
    pub h_resolution: i32,   // Pixels par mètre (horizontal)
    pub v_resolution: i32,   // Pixels par mètre (vertical)
    pub colors_used: u32,    // Nombre de couleurs dans la palette (0 = tous)
    pub colors_important: u32, // Nombre de couleurs importantes (0 = tous)
}

/// Représente une image BMP chargée en mémoire
#[derive(Debug)]
pub struct BmpImage {
    pub file_header: BmpFileHeader,
    pub dib_header: BmpDibHeader,
    pub pixels: Vec<u8>,     // Les données brutes des pixels
}

// ============================================================================
// ÉTAPE 2 : Fonctions helper pour lire les types primitifs
// ============================================================================

/// Lit un u16 en little-endian (format BMP standard)
fn read_u16_le<R: Read>(reader: &mut R) -> std::io::Result<u16> {
    let mut buf = [0; 2];
    reader.read_exact(&mut buf)?;
    Ok(u16::from_le_bytes(buf))
}

/// Lit un u32 en little-endian (format BMP standard)
fn read_u32_le<R: Read>(reader: &mut R) -> std::io::Result<u32> {
    let mut buf = [0; 4];
    reader.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes(buf))
}

/// Lit un i32 en little-endian (format BMP standard)
fn read_i32_le<R: Read>(reader: &mut R) -> std::io::Result<i32> {
    let mut buf = [0; 4];
    reader.read_exact(&mut buf)?;
    Ok(i32::from_le_bytes(buf))
}

// ============================================================================
// ÉTAPE 3 : Parsage du File Header (14 bytes)
// ============================================================================

fn parse_file_header<R: Read>(reader: &mut R) -> std::io::Result<BmpFileHeader> {
    // Bytes 0-1 : "BM" (signature)
    let mut signature = [0; 2];
    reader.read_exact(&mut signature)?;
    if signature != [b'B', b'M'] {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Pas une signature BMP valide",
        ));
    }
    
    // Bytes 2-5 : file_size (u32 little-endian)
    let file_size = read_u32_le(reader)?;
    
    // Bytes 6-9 : reserved (on l'ignore)
    read_u32_le(reader)?;
    
    // Bytes 10-13 : pixel_offset (u32 little-endian)
    let pixel_offset = read_u32_le(reader)?;
    
    Ok(BmpFileHeader {
        file_size,
        pixel_offset,
    })
}

// ============================================================================
// ÉTAPE 4 : Parsage du DIB Header (40 bytes)
// ============================================================================

fn parse_dib_header<R: Read>(reader: &mut R) -> std::io::Result<BmpDibHeader> {
    // Bytes 0-3 : header_size (on le lit mais on l'ignore pour cette version)
    let _header_size = read_u32_le(reader)?;
    
    // Bytes 4-7 : width
    let width = read_i32_le(reader)?;
    
    // Bytes 8-11 : height
    let height = read_i32_le(reader)?;
    
    // Bytes 12-13 : planes (doit être 1)
    let planes = read_u16_le(reader)?;
    if planes != 1 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Le nombre de planes doit être 1",
        ));
    }
    
    // Bytes 14-15 : bits_per_pixel
    let bits_per_pixel = read_u16_le(reader)?;
    
    // Bytes 16-19 : compression
    let compression = read_u32_le(reader)?;
    if compression != 0 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Seule la compression 0 (aucune) est supportée",
        ));
    }
    
    // Bytes 20-23 : image_size
    let image_size = read_u32_le(reader)?;
    
    // Bytes 24-27 : h_resolution
    let h_resolution = read_i32_le(reader)?;
    
    // Bytes 28-31 : v_resolution
    let v_resolution = read_i32_le(reader)?;
    
    // Bytes 32-35 : colors_used
    let colors_used = read_u32_le(reader)?;
    
    // Bytes 36-39 : colors_important
    let colors_important = read_u32_le(reader)?;
    
    Ok(BmpDibHeader {
        width,
        height,
        planes,
        bits_per_pixel,
        compression,
        image_size,
        h_resolution,
        v_resolution,
        colors_used,
        colors_important,
    })
}

// ============================================================================
// ÉTAPE 5 : Fonction principale pour charger un BMP
// ============================================================================

pub fn load_bmp<P: AsRef<Path>>(path: P) -> std::io::Result<BmpImage> {
    let file = std::fs::File::open(path)?;
    let mut reader = std::io::BufReader::new(file);
    
    // Parse file header
    let file_header = parse_file_header(&mut reader)?;
    println!("File header parsé: {:?}", file_header);
    
    // Parse DIB header
    let dib_header = parse_dib_header(&mut reader)?;
    println!("DIB header parsé: {:?}", dib_header);
    
    // Saute à l'offset des pixels
    reader.seek(SeekFrom::Start(file_header.pixel_offset as u64))?;
    
    // Lit les données des pixels
    let mut pixels = vec![0; dib_header.image_size as usize];
    reader.read_exact(&mut pixels)?;
    
    Ok(BmpImage {
        file_header,
        dib_header,
        pixels,
    })
}

// ============================================================================
// ÉTAPE 6 : Fonction pour extraire les données RGB
// ============================================================================

/// Convertit les pixels BMP en format RGB standard
pub fn bmp_to_rgba(image: &BmpImage) -> Result<Vec<u8>, String> {
    let bits_per_pixel = image.dib_header.bits_per_pixel;
    
    if bits_per_pixel != 24 && bits_per_pixel != 32 {
        return Err(format!(
            "Format {} bits pas encore implémenté",
            bits_per_pixel
        ));
    }
    
    let width = image.dib_header.width as usize;
    let height = image.dib_header.height.abs() as usize;
    let bytes_per_pixel = (bits_per_pixel / 8) as usize;
    
    let mut rgba = Vec::with_capacity(width * height * 4);
    
    // BMP stocke les lignes de bas en haut (sauf si height est négatif)
    for row in 0..height {
        for col in 0..width {
            let pixel_index = if image.dib_header.height > 0 {
                // Image droite à gauche (standard)
                ((height - 1 - row) * width + col) * bytes_per_pixel
            } else {
                // Image à l'endroit
                (row * width + col) * bytes_per_pixel
            };
            
            if pixel_index + bytes_per_pixel <= image.pixels.len() {
                let b = image.pixels[pixel_index];
                let g = image.pixels[pixel_index + 1];
                let r = image.pixels[pixel_index + 2];
                let a = if bytes_per_pixel == 4 {
                    image.pixels[pixel_index + 3]
                } else {
                    255
                };
                
                rgba.push(r);
                rgba.push(g);
                rgba.push(b);
                rgba.push(a);
            }
        }
    }
    
    Ok(rgba)
}

// ============================================================================
// ÉTAPE 7 : Fonction utilitaire pour charger et convertir en une étape
// ============================================================================

/// Structure pour retourner les données de texture directement utilisables
#[derive(Debug)]
pub struct TextureData {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>, // RGBA format
}

/// Charge un fichier BMP et retourne les données au format RGBA
/// C'est la fonction qu'on va appeler depuis texture.rs
pub fn load_bmp_as_texture<P: AsRef<Path>>(path: P) -> Result<TextureData, String> {
    // Charge et parse le BMP
    let image = load_bmp(&path)
        .map_err(|e| format!("Erreur lors du chargement du BMP: {}", e))?;
    
    println!(
        "BMP chargé: {}x{} - {} bits/pixel",
        image.dib_header.width, image.dib_header.height, image.dib_header.bits_per_pixel
    );
    
    // Convertit en RGBA
    let pixels = bmp_to_rgba(&image)?;
    
    Ok(TextureData {
        width: image.dib_header.width as u32,
        height: image.dib_header.height.abs() as u32,
        pixels,
    })
}
