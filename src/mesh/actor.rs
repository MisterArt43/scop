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
}
