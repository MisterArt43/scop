use std::ffi::c_void;

use gl::{
    types::{GLenum, GLint, GLuint},
    ActiveTexture, BindTexture, DeleteTextures, GenTextures, GenerateMipmap, TexImage2D,
    TexParameteri,
};

#[derive(Debug, Clone, Copy)]
pub enum TextureFormat {
    R8,
    RG8,
    RGB8,
    RGBA8,
}

impl TextureFormat {
    fn internal_format(self) -> GLint {
        match self {
            Self::R8 => gl::R8 as GLint,
            Self::RG8 => gl::RG8 as GLint,
            Self::RGB8 => gl::RGB8 as GLint,
            Self::RGBA8 => gl::RGBA8 as GLint,
        }
    }

    fn format(self) -> GLenum {
        match self {
            Self::R8 => gl::RED,
            Self::RG8 => gl::RG,
            Self::RGB8 => gl::RGB,
            Self::RGBA8 => gl::RGBA,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TextureFilter {
    Nearest,
    Linear,
    NearestMipmapNearest,
    LinearMipmapNearest,
    NearestMipmapLinear,
    LinearMipmapLinear,
}

impl TextureFilter {
    fn to_gl(self) -> GLint {
        match self {
            Self::Nearest => gl::NEAREST as GLint,
            Self::Linear => gl::LINEAR as GLint,

            Self::NearestMipmapNearest => gl::NEAREST_MIPMAP_NEAREST as GLint,

            Self::LinearMipmapNearest => gl::LINEAR_MIPMAP_NEAREST as GLint,

            Self::NearestMipmapLinear => gl::NEAREST_MIPMAP_LINEAR as GLint,

            Self::LinearMipmapLinear => gl::LINEAR_MIPMAP_LINEAR as GLint,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TextureWrap {
    Repeat,
    MirroredRepeat,
    ClampToEdge,
    ClampToBorder,
}

impl TextureWrap {
    fn to_gl(self) -> GLint {
        match self {
            Self::Repeat => gl::REPEAT as GLint,
            Self::MirroredRepeat => gl::MIRRORED_REPEAT as GLint,
            Self::ClampToEdge => gl::CLAMP_TO_EDGE as GLint,
            Self::ClampToBorder => gl::CLAMP_TO_BORDER as GLint,
        }
    }
}

#[derive(Debug)]
pub struct Texture {
    id: GLuint,
    target: GLenum,
    width: u32,
    height: u32,
}

impl Texture {
    /*================== CONSTRUCTOR ==================*/

    pub fn new_2d() -> Self {
        let mut id = 0;

        unsafe {
            GenTextures(1, &mut id);
        }

        Self {
            id,
            target: gl::TEXTURE_2D,
            width: 0,
            height: 0,
        }
    }

    /*================== BIND ==================*/

    pub fn bind(&self, unit: u32) {
        unsafe {
            ActiveTexture(gl::TEXTURE0 + unit);
            BindTexture(self.target, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            BindTexture(self.target, 0);
        }
    }

    /*================== UPLOAD ==================*/

    pub fn upload(
        &mut self,
        width: u32,
        height: u32,
        pixels: &[u8],
        format: TextureFormat,
    ) -> &mut Self {
        self.width = width;
        self.height = height;

        self.bind(0);

        unsafe {
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);

            TexImage2D(
                self.target,
                0,
                format.internal_format(),
                width as i32,
                height as i32,
                0,
                format.format(),
                gl::UNSIGNED_BYTE,
                pixels.as_ptr() as *const c_void,
            );
        }

        self.unbind();

        self
    }

    /*================== FILTER ==================*/

    pub fn set_filter(&self, min_filter: TextureFilter, mag_filter: TextureFilter) -> &Self {
        self.bind(0);

        unsafe {
            TexParameteri(self.target, gl::TEXTURE_MIN_FILTER, min_filter.to_gl());

            TexParameteri(self.target, gl::TEXTURE_MAG_FILTER, mag_filter.to_gl());
        }

        self.unbind();

        &self
    }

    /*================== WRAP ==================*/

    pub fn set_wrap(&self, wrap_s: TextureWrap, wrap_t: TextureWrap) -> &Self {
        self.bind(0);

        unsafe {
            TexParameteri(self.target, gl::TEXTURE_WRAP_S, wrap_s.to_gl());

            TexParameteri(self.target, gl::TEXTURE_WRAP_T, wrap_t.to_gl());
        }

        self.unbind();

        &self
    }

    /*================== MIPMAPS ==================*/

    pub fn generate_mipmaps(&self) -> &Self {
        self.bind(0);

        unsafe {
            GenerateMipmap(self.target);
        }

        self.unbind();

        &self
    }

    /*================== GETTERS ==================*/

    pub fn id(&self) -> GLuint {
        self.id
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        if self.id != 0 {
            unsafe {
                DeleteTextures(1, &self.id);
            }
        }
    }
}
