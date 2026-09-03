use crate::gfx::texture::Texture;
use crate::math::vec3::Vec3;
use crate::shader::Shader;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Material {
    _name: String,

    ambient: Vec3,
    specular: Vec3,
    diffuse: Vec3,
    
    shininess: f32,
    opacity: f32,

    diffuse_texture: Option<Rc<Texture>>,
    specular_texture: Option<Rc<Texture>>,
    // normal_map: Option<Rc<Texture>>,
}

impl Material {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            _name: name.into(),

            ambient: Vec3::ZERO,
            diffuse: Vec3::ONE,
            specular: Vec3::ZERO,

            shininess: 32.0,
            opacity: 1.0,

            diffuse_texture: None,
            specular_texture: None,
        }
    }

    /**
     * * Will activate the shader and bind the textures to the appropriate texture units.
     * * * Will also set the material properties in the shader uniforms.
     * * * Note: end with a Vao bind to draw the mesh with the material applied.
     */
    pub fn apply(&self, shader: &Shader) {
        shader.activate();
        // Bind textures to texture units
        if let Some(ref tex) = self.diffuse_texture {
            tex.bind(0); // GL_TEXTURE0
            shader.set_uniform_int("texture1", 0);
        }
        if let Some(ref tex) = self.specular_texture {
            tex.bind(1); // GL_TEXTURE1
            shader.set_uniform_int("texture2", 1);
        }
        // Set material uniforms
        shader.set_uniform_vec3("material.ambient", &self.ambient.to_array());
        shader.set_uniform_vec3("material.diffuse", &self.diffuse.to_array());
        shader.set_uniform_vec3("material.specular", &self.specular.to_array());
        shader.set_uniform_float("material.shininess", self.shininess);
        shader.set_uniform_float("material.opacity", self.opacity);
    }

    pub fn from_data(
        name: impl Into<String>,
        ambient: Vec3,
        diffuse: Vec3,
        specular: Vec3,
        shininess: f32,
        opacity: f32,
        diffuse_texture: Option<Rc<Texture>>,
        specular_texture: Option<Rc<Texture>>,
    ) -> Self {
        Self {
            _name: name.into(),
            ambient,
            diffuse,
            specular,
            shininess,
            opacity,
            diffuse_texture,
            specular_texture,
        }
    }
}
