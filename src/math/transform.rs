use crate::math::{quaternion::Quaternion, vec3::Vec3};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quaternion,
    pub scale: Vec3,
}

impl Transform {
    pub fn new() -> Self {
        Transform {
            position: Vec3::new(0.0, 0.0, 0.0),
            rotation: Quaternion::new(0.0, 0.0, 0.0, 1.0),
            scale: Vec3::new(1.0, 1.0, 1.0),
        }
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }

    pub fn set_rotation(&mut self, rotation: Quaternion) {
        self.rotation = rotation;
    }

    pub fn set_scale(&mut self, scale: Vec3) {
        self.scale = scale;
    }

    pub fn get_position(self) -> Vec3 {
        self.position
    }

    pub fn get_rotation(self) -> Quaternion {
        self.rotation
    }

    pub fn get_scale(self) -> Vec3 {
        self.scale
    }

    pub fn translate(&mut self, translation: Vec3) {
        self.position = self.position + translation;
    }

    pub fn rotate(&mut self, rotation: Quaternion) {
        self.rotation = self.rotation.mul(&rotation);
    }

    pub fn scale(&mut self, scale: Vec3) {
        self.scale = self.scale.mul_vec(scale);
    }

    pub fn reset(&mut self) {
        self.position = Vec3::new(0.0, 0.0, 0.0);
        self.rotation = Quaternion::new(0.0, 0.0, 0.0, 1.0);
        self.scale = Vec3::new(1.0, 1.0, 1.0);
    }

    pub fn lerp(self, other: &Transform, t: f32) -> Transform {
        let t = t.clamp(0.0, 1.0);

        Transform {
            position: self.position * (1.0 - t) + other.position * t,

            rotation: (self
                .rotation
                .mul_f32(1.0 - t)
                .add(&other.rotation.mul_f32(t)))
            .normalize(),

            scale: self.scale * (1.0 - t) + other.scale * t,
        }
    }
}
