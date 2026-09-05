use crate::math::vec3::Vec3;

pub enum ViewMode {
    Free,

    LookAt { target: Vec3, up: Vec3 },
}
