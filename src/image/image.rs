#[derive(Debug, Clone)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub channels: u8,
    pub pixels: Vec<u8>,
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

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn channels(&self) -> u8 {
        self.channels
    }

    pub fn pixels(&self) -> &[u8] {
        &self.pixels
    }
}
