use crate::{shader::Shader, texture::Texture};

pub struct Material {
    diffuse_texture: Option<Texture>,
    specular_texture: Option<Texture>,
    normal_map: Option<Texture>,
    // material properties
    ambient: [f32; 3],
    diffuse: [f32; 3],
    specular: [f32; 3],
    shininess: f32,
    name: String,
}

impl Material {
    pub fn apply(&self, shader: &Shader) {
        shader.activate();
        // Bind textures to texture units
        if let Some(ref tex) = self.diffuse_texture {
            tex.bind(0); // GL_TEXTURE0
            shader.set_uniform_int("material.diffuse", 0);
        }
        if let Some(ref tex) = self.specular_texture {
            tex.bind(1); // GL_TEXTURE1
            shader.set_uniform_int("material.specular", 1);
        }
        // Set material uniforms
        shader.set_uniform_vec3("material.ambient", &self.ambient);
        shader.set_uniform_float("material.shininess", self.shininess);
    }
}
