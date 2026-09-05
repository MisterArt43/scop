use crate::math::mat4::Mat4;

#[derive(Debug, Copy, Clone)]
pub enum Projection {
    Perspective {
        fov_y: f32,
        aspect_ratio: f32,
        near: f32,
        far: f32,
    },

    Orthographic {
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    },
}

impl Projection {
    pub fn perspective(fov_y: f32, aspect_ratio: f32, near: f32, far: f32) -> Self {
        Self::Perspective {
            fov_y,
            aspect_ratio,
            near,
            far,
        }
    }

    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        Self::Orthographic {
            left,
            right,
            bottom,
            top,
            near,
            far,
        }
    }

    pub fn matrix(&self) -> Mat4 {
        match *self {
            Projection::Perspective {
                fov_y,
                aspect_ratio,
                near,
                far,
            } => Mat4::perspective(fov_y, aspect_ratio, near, far),

            Projection::Orthographic {
                left,
                right,
                bottom,
                top,
                near,
                far,
            } => Mat4::orthographic(left, right, bottom, top, near, far),
        }
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        if let Projection::Perspective {
            aspect_ratio: current_aspect,
            ..
        } = self
        {
            *current_aspect = aspect_ratio;
        }
    }
}
