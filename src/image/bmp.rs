use std::{fs, path::Path};

use anyhow::{bail, Result};

use super::image::Image;

pub struct BMP;

impl BMP {
    pub fn load(path: &Path) -> Result<Image> {
        let data = fs::read(path)?;

        if data.len() < 54 || &data[..2] != b"BM" {
            bail!("Invalid BMP");
        }

        let offset = u32::from_le_bytes(data[10..14].try_into()?) as usize;
        let width = i32::from_le_bytes(data[18..22].try_into()?);
        let height = i32::from_le_bytes(data[22..26].try_into()?);
        let bpp = u16::from_le_bytes(data[28..30].try_into()?);
        let compression = u32::from_le_bytes(data[30..34].try_into()?);

        if width <= 0 || height == 0 {
            bail!("Invalid BMP size");
        }

        if !matches!((bpp, compression), (24, 0) | (32, 0) | (32, 3)) {
            bail!("Unsupported BMP format");
        }

        let w = width as usize;
        let h = height.unsigned_abs() as usize;
        let channels = (bpp / 8) as usize;
        let row_size = (w * channels + 3) & !3;

        if offset + row_size * h > data.len() {
            bail!("Invalid BMP data");
        }

        let mut pixels = vec![0; w * h * channels];

        for y in 0..h {
            let sy = if height > 0 { h - 1 - y } else { y };

            for x in 0..w {
                let src = offset + sy * row_size + x * channels;
                let dst = (y * w + x) * channels;

                pixels[dst] = data[src + 2];
                pixels[dst + 1] = data[src + 1];
                pixels[dst + 2] = data[src];

                if channels == 4 {
                    pixels[dst + 3] = data[src + 3];
                }
            }
        }

        Ok(Image {
            width: w as u32,
            height: h as u32,
            channels: channels as u8,
            pixels,
        })
    }
}
