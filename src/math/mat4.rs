use std::ops::Mul;

use crate::math::{quaternion::Quaternion, vec3::Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Mat4 {
    pub(crate) data: [[f32; 4]; 4],
}

impl Mat4 {
    pub fn new() -> Self {
        Mat4 {
            data: [[0.0; 4]; 4],
        }
    }

    pub fn identity() -> Self {
        let mut m = Mat4::new();
        for i in 0..4 {
            m.data[i][i] = 1.0;
        }
        m
    }

    pub fn mul(self, other: &Mat4) -> Mat4 {
        let mut result = Mat4::new();
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result.data[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }
        result
    }

    pub fn transpose(self) -> Mat4 {
        let mut result = Mat4::new();
        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = self.data[j][i];
            }
        }
        result
    }

    pub fn inverse(self) -> Option<Mat4> {
        // Inversion d'une matrice 4x4 (implémentation simplifiée, pas optimisée)
        let mut inv = Mat4::new();
        let mut det: f32;

        inv.data[0][0] = self.data[1][1] * self.data[2][2] * self.data[3][3]
            - self.data[1][1] * self.data[2][3] * self.data[3][2]
            - self.data[2][1] * self.data[1][2] * self.data[3][3]
            + self.data[2][1] * self.data[1][3] * self.data[3][2]
            + self.data[3][1] * self.data[1][2] * self.data[2][3]
            - self.data[3][1] * self.data[1][3] * self.data[2][2];

        det = self.data[0][0] * inv.data[0][0];
        if det == 0.0 {
            return None;
        }

        det = 1.0 / det;

        for i in 0..4 {
            for j in 0..4 {
                inv.data[i][j] *= det;
            }
        }

        Some(inv)
    }

    // https://www.songho.ca/opengl/gl_camera.html
    pub fn look_at(eye: Vec3, target: Vec3, up: Vec3) -> Mat4 {
        let f = (target - eye).normalize();
        let s = f.cross(up).normalize();
        let u = s.cross(f);

        Mat4 {
            data: [
                [s.x, u.x, -f.x, 0.0],
                [s.y, u.y, -f.y, 0.0],
                [s.z, u.z, -f.z, 0.0],
                [-s.dot(eye), -u.dot(eye), f.dot(eye), 1.0],
            ],
        }
    }

    pub fn as_ptr(self) -> *const f32 {
        self.data.as_ptr() as *const f32
    }

    pub fn to_flat_array(self) -> [f32; 16] {
        // Store as-is (row-major) - OpenGL will interpret as column-major
        // because we use GL_FALSE in glUniformMatrix4fv
        let mut out = [0.0f32; 16];
        for i in 0..4 {
            for j in 0..4 {
                out[i * 4 + j] = self.data[i][j];
            }
        }
        out
    }

    pub fn perspective(fov_deg: f32, aspect: f32, near: f32, far: f32) -> Mat4 {
        let fov_rad = fov_deg.to_radians();
        let f = 1.0 / (fov_rad / 2.0).tan();
        let nf = 1.0 / (near - far);

        let mut m = Mat4::new();
        m.data[0][0] = f / aspect;
        m.data[1][1] = f;
        m.data[2][2] = (far + near) * nf;
        m.data[2][3] = -1.0;
        m.data[3][2] = 2.0 * far * near * nf;
        m
    }

    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Mat4 {
        let rl = 1.0 / (right - left);
        let tb = 1.0 / (top - bottom);
        let fn_ = 1.0 / (far - near);

        let mut m = Mat4::new();
        m.data[0][0] = 2.0 * rl;
        m.data[1][1] = 2.0 * tb;
        m.data[2][2] = -2.0 * fn_;
        m.data[3][0] = -(right + left) * rl;
        m.data[3][1] = -(top + bottom) * tb;
        m.data[3][2] = -(far + near) * fn_;
        m.data[3][3] = 1.0;
        m
    }

    pub fn translation(translation: Vec3) -> Mat4 {
        let mut m = Mat4::identity();
        m.data[3][0] = translation.x;
        m.data[3][1] = translation.y;
        m.data[3][2] = translation.z;
        m
    }

    pub fn scaling(scale: Vec3) -> Mat4 {
        let mut m = Mat4::identity();
        m.data[0][0] = scale.x;
        m.data[1][1] = scale.y;
        m.data[2][2] = scale.z;
        m
    }
}

impl From<Quaternion> for Mat4 {
    fn from(q: Quaternion) -> Self {
        q.to_rotation_matrix()
    }
}

impl Mul<Mat4> for Mat4 {
    type Output = Mat4;

    fn mul(self, other: Mat4) -> Self::Output {
        self.mul(&other)
    }
}
