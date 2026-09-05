use crate::{asset::MtlMaterial, math::transform::Transform, mesh::Mesh};

#[derive(Debug)]
pub struct SubMesh {
    pub object: String,
    pub group: String,
    pub material: String,
    pub material_data: Option<MtlMaterial>,
    pub mesh: Mesh,
    pub transform: Transform,
}

impl SubMesh {
    pub fn new(
        object: String,
        group: String,
        material: String,
        material_data: Option<MtlMaterial>,
        mesh: Mesh,
    ) -> Self {
        Self {
            object,
            group,
            material,
            material_data,
            mesh,
            transform: Transform::new(),
        }
    }

    pub fn draw(&self) {
        self.mesh.draw();
    }
}
