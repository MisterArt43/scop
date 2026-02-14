use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[allow(unused)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[allow(unused)]
pub struct Texture {
    file_path: String,
    width: u32,
    height: u32,
    pixels: Vec<Pixel>,
}

/**
 * This implementation will parse different image formats
 * and create a texture object.
 */
impl Texture {
    pub fn new(path: &str) -> Texture {
        let mut texture = Texture {
            file_path: path.to_string(),
            width: 0,
            height: 0,
            pixels: Vec::new(),
        };

        match find_image_format(path) {
            Some("ppm") => {
                texture.pixels = parse_ppm(path);
            }
            Some("bmp") => {
                // parse bmp
            }
            _ => panic!("Unsupported image format"),
        }

        texture
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

#[allow(unused_variables)]
#[allow(unused_mut)]
fn parse_ppm(path: &str) -> Vec<Pixel> {
    // open file, read header, read pixel data
    let file = File::open(path).expect("Failed to open PPM file");
    let mut reader = BufReader::new(file);
    let mut header = String::new();
    reader
        .read_line(&mut header)
        .expect("Failed to read PPM header");

        let mut width = 0;
        let mut height = 0;
        let mut max_color = 0;
        let mut format = String::new();
    
    
    Vec::new()
}
