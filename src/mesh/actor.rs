use crate::{
    math::transform::Transform,
    mesh::{parser, SubMesh},
};

#[derive(Debug)]
pub struct Actor {
    pub submeshes: Vec<SubMesh>,
    pub transform: Transform,
}

impl Actor {
    pub fn new(submeshes: Vec<SubMesh>, transform: Option<Transform>) -> Self {
        Self {
            submeshes,
            transform: transform.unwrap_or_else(Transform::new),
        }
    }

    pub fn from_obj(path: &str) -> Result<Self, String> {
        let submeshes = parser::parse_obj(path)?;

        Ok(Self::new(submeshes, None))
    }

    pub fn model_matrix(&self) -> crate::math::mat4::Mat4 {
        let translation = crate::math::mat4::Mat4::translation(self.transform.position);
        let rotation = self.transform.get_mat4_rotation();
        let scale = crate::math::mat4::Mat4::scaling(self.transform.scale);
        translation * rotation * scale
    }
}
