use std::{fs, path::Path};

use anyhow::{anyhow, bail, Result};

use crate::image::image::Image;

pub struct PPM;

impl PPM {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Image> {
        let data = fs::read(path)?;
        let mut pos = 0;

        let format = next_token(&data, &mut pos)?;
        let width = next_token(&data, &mut pos)?.parse::<u32>()?;
        let height = next_token(&data, &mut pos)?.parse::<u32>()?;
        let max_value = next_token(&data, &mut pos)?.parse::<u32>()?;

        if width == 0 || height == 0 {
            bail!("Invalid PPM size");
        }

        if max_value == 0 || max_value > 255 {
            bail!("Only PPM max values from 1 to 255 are supported");
        }

        let count = width as usize * height as usize * 3;

        let mut image = match format.as_str() {
            "P3" => parse_p3(&data, &mut pos, count, max_value)?,
            "P6" => parse_p6(&data, &mut pos, count, max_value)?,
            _ => bail!("Unsupported PPM format: {format}"),
        };

        image.width = width;
        image.height = height;

        Ok(image)
    }
}

fn parse_p3(data: &[u8], pos: &mut usize, count: usize, max_value: u32) -> Result<Image> {
    let mut pixels = Vec::with_capacity(count);

    for _ in 0..count {
        let value = next_token(data, pos)?.parse::<u32>()?;

        if value > max_value {
            bail!("Pixel value exceeds PPM max value");
        }

        pixels.push((value * 255 / max_value) as u8);
    }

    Ok(Image {
        width: 0,
        height: 0,
        channels: 3,
        pixels,
    })
}

fn parse_p6(data: &[u8], pos: &mut usize, count: usize, max_value: u32) -> Result<Image> {
    // Skip the separator between the header and binary pixels.
    if *pos < data.len() && data[*pos] == b'\r' {
        *pos += 1;
    }

    if *pos < data.len() && data[*pos] == b'\n' {
        *pos += 1;
    } else if *pos < data.len() && data[*pos].is_ascii_whitespace() {
        *pos += 1;
    }

    if data.len() < *pos + count {
        bail!("Incomplete P6 pixel data");
    }

    let pixels = &data[*pos..*pos + count];

    if max_value == 255 {
        return Ok(Image {
            width: 0,
            height: 0,
            channels: 3,
            pixels: pixels.to_vec(),
        });
    }

    Ok(Image {
        width: 0,
        height: 0,
        channels: 3,
        pixels: pixels
            .iter()
            .map(|&value| (value as u32 * 255 / max_value) as u8)
            .collect(),
    })
}

fn next_token(data: &[u8], pos: &mut usize) -> Result<String> {
    loop {
        while *pos < data.len() && data[*pos].is_ascii_whitespace() {
            *pos += 1;
        }

        if *pos < data.len() && data[*pos] == b'#' {
            while *pos < data.len() && data[*pos] != b'\n' {
                *pos += 1;
            }

            continue;
        }

        break;
    }

    if *pos >= data.len() {
        return Err(anyhow!("Unexpected end of PPM file"));
    }

    let start = *pos;

    while *pos < data.len() && !data[*pos].is_ascii_whitespace() && data[*pos] != b'#' {
        *pos += 1;
    }

    Ok(std::str::from_utf8(&data[start..*pos])?.to_string())
}
