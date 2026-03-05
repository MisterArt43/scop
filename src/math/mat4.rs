use crate::math::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Mat4 {
    data: [[f32; 4]; 4],
}

impl Mat4 {
    pub fn new() -> Self {
        Mat4 { data: [[0.0; 4]; 4] }
    }

    pub fn identity() -> Self {
        let mut m = Mat4::new();
        for i in 0..4 {
            m.data[i][i] = 1.0;
        }
        m
    }

    pub fn mul(&self, other: &Mat4) -> Mat4 {
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

    pub fn transpose(&self) -> Mat4 {
        let mut result = Mat4::new();
        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = self.data[j][i];
            }
        }
        result
    }

    pub fn inverse(&self) -> Option<Mat4> {
        // Inversion d'une matrice 4x4 (implémentation simplifiée, pas optimisée)
        let mut inv = Mat4::new();
        let mut det: f32;

        inv.data[0][0] = self.data[1][1] * self.data[2][2] * self.data[3][3] -
                         self.data[1][1] * self.data[2][3] * self.data[3][2] -
                         self.data[2][1] * self.data[1][2] * self.data[3][3] +
                         self.data[2][1] * self.data[1][3] * self.data[3][2] +
                         self.data[3][1] * self.data[1][2] * self.data[2][3] -
                         self.data[3][1] * self.data[1][3] * self.data[2][2];

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
    pub fn look_at(eye: Vec3, target: Vec3, up_dir: Vec3) -> Mat4 {
        let forward = eye.sub(&target).normalize();
        let left = up_dir.cross(&forward).normalize();
        let up = forward.cross(&left);

        Mat4 {
            data: [
                [left.x, up.x, -forward.x, 0.0],
                [left.y, up.y, -forward.y, 0.0],
                [left.z, up.z, -forward.z, 0.0],
                [-left.dot(&eye), -up.dot(&eye), forward.dot(&eye), 1.0],
            ],
        }
    }
}