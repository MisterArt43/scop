use std::path::{Path, PathBuf};

pub const MISSING_TEXTURE: &str = "core/assets/missing_texture.bmp";

pub fn missing_texture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(MISSING_TEXTURE)
}
