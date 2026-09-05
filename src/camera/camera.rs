use crate::math::{mat4::Mat4, transform::Transform, vec3::Vec3};

use super::{projection::Projection, view::ViewMode};

#[derive(Debug, Copy, Clone)]
pub struct CameraBasis {
    pub forward: Vec3,
    pub right: Vec3,
    pub up: Vec3,
}

pub struct Camera {
    pub transform: Transform,
    pub projection: Projection,
    pub view_mode: ViewMode,
}

impl Camera {
    pub fn new(transform: Transform, projection: Projection) -> Self {
        Self {
            transform,
            projection,
            view_mode: ViewMode::Free,
        }
    }

    pub fn basis(&self) -> CameraBasis {
        match &self.view_mode {
            ViewMode::Free => {
                let forward = self
                    .transform
                    .rotation
                    .rotate_vector(Vec3::new(0.0, 0.0, -1.0))
                    .normalize();

                let right = self
                    .transform
                    .rotation
                    .rotate_vector(Vec3::new(1.0, 0.0, 0.0))
                    .normalize();

                let up = right.cross(forward).normalize();

                CameraBasis { forward, right, up }
            }

            ViewMode::LookAt { target, up } => {
                let forward = (*target - self.transform.position).normalize();

                let right = forward.cross(*up).normalize();

                let up = right.cross(forward).normalize();

                CameraBasis { forward, right, up }
            }
        }
    }

    pub fn forward(&self) -> Vec3 {
        self.basis().forward
    }

    pub fn right(&self) -> Vec3 {
        self.basis().right
    }

    pub fn up(&self) -> Vec3 {
        self.basis().up
    }

    fn free_view_mat(&self) -> Mat4 {
        let inverse_rotation = self.transform.rotation.conjugate();

        Mat4::from(inverse_rotation) * Mat4::translation(-self.transform.position)
    }

    pub fn view_mat(&self) -> Mat4 {
        match &self.view_mode {
            ViewMode::Free => self.free_view_mat(),

            ViewMode::LookAt { target, up } => Mat4::look_at(self.transform.position, *target, *up),
        }
    }

    pub fn projection_mat(&self) -> Mat4 {
        self.projection.matrix()
    }

    pub fn view_projection_mat(&self) -> Mat4 {
        self.projection_mat() * self.view_mat()
    }
}
