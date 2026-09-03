use std::path::PathBuf;

pub const MISSING_TEXTURE: &str = "core/assets/missing_texture.bmp";

pub fn missing_texture_path() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|path| path.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."))
        .join(MISSING_TEXTURE)
}