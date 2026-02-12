use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

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

fn parse_ppm(path: &str) -> Vec<Pixel> {
    // open file, read header, read pixel data
    let file = File::open(path).expect("Failed to open PPM file");
    let mut reader = BufReader::new(file);
    let mut header = String::new();
    reader
        .read_line(&mut header)
        .expect("Failed to read PPM header");

    //check if header is valid and if its P3 or P6
    // if header.trim() != "P3" && header.trim() != "P6" {
    //     panic!("Unsupported PPM format");
    // }

    // //skip comments and read width, height, max color value
    // let mut width = 0;
    // let mut height = 0;
    // let mut max_color = 0;

    // loop {
    //     let mut line = String::new();
    //     reader
    //         .read_line(&mut line)
    //         .expect("Failed to read PPM header");
    //     if line.starts_with("#") {
    //         continue; // skip comments
    //     }
    //     let parts: Vec<&str> = line.trim().split_whitespace().collect();
    //     if parts.len() == 2 {
    //         width = parts[0].parse::<u32>().expect("Failed to parse width");
    //         height = parts[1].parse::<u32>().expect("Failed to parse height");
    //     } else if parts.len() == 1 {
    //         max_color = parts[0]
    //             .parse::<u32>()
    //             .expect("Failed to parse max color value");
    //         break;
    //     }
    // }

    //redo parsing with SOME for P3/ P6
        let mut width = 0;
        let mut height = 0;
        let mut max_color = 0;
        let mut format = String::new();
    
        loop {
            let mut line = String::new();
            reader
                .read_line(&mut line)
                .expect("Failed to read PPM header");
            if line.starts_with("#") {
                continue; // skip comments
            }
            if format.is_empty() {
                format = line.trim().to_string();
                continue;
            }
            let parts: Vec<&str> = line.trim().split_whitespace().collect();
            if parts.len() == 2 {
                width = parts[0].parse::<u32>().expect("Failed to parse width");
                height = parts[1].parse::<u32>().expect("Failed to parse height");
            } else if parts.len() == 1 {
                max_color = parts[0]
                    .parse::<u32>()
                    .expect("Failed to parse max color value");
                break;
            }
        }
    
        //read pixel data based on format
        match format.as_str() {
            "P3" => {
                // read ASCII pixel data
                let mut pixels = Vec::new();
                for line in reader.lines() {
                    let line = line.expect("Failed to read pixel data");
                    for part in line.trim().split_whitespace() {
                        let value = part.parse::<u8>().expect("Failed to parse pixel value");
                        pixels.push(value);
                    }
                }
                // convert flat pixel data to Pixel structs
                pixels
                    .chunks(3)
                    .map(|chunk| Pixel {
                        r: chunk[0],
                        g: chunk[1],
                        b: chunk[2],
                        a: 255,
                    })
                    .collect()
            }
            "P6" => {
                // read binary pixel data
                let mut pixels = Vec::new();
                reader.read_to_end(&mut pixels).expect("Failed to read pixel data");
                // convert flat pixel data to Pixel structs
                pixels
                    .chunks(3)
                    .map(|chunk| Pixel {
                        r: chunk[0],
                        g: chunk[1],
                        b: chunk[2],
                        a: 255,
                    })
                    .collect()
            }
            _ => panic!("Unsupported PPM format"),
        }

    Vec::new()
}
