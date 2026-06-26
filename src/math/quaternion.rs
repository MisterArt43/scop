use crate::math::mat4::Mat4;
use crate::math::vec3::{FORWARD, RIGHT, UP, Vec3};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quaternion {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Quaternion { x, y, z, w }
    }

    pub fn identity() -> Self {
        Quaternion {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }

    pub fn mul_f32(&self, other: f32) -> Quaternion {
        Quaternion {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }

    pub fn mul(&self, other: &Quaternion) -> Quaternion {
        Quaternion {
            x: self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y,
            y: self.w * other.y - self.x * other.z + self.y * other.w + self.z * other.x,
            z: self.w * other.z + self.x * other.y - self.y * other.x + self.z * other.w,
            w: self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z,
        }
    }

    pub fn add(&self, other: &Quaternion) -> Quaternion {
        Quaternion {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }

    pub fn add_f32(&self, other: f32) -> Quaternion {
        Quaternion {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
            w: self.w + other,
        }
    }

    pub fn dot(&self, other: &Quaternion) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn div(&self, scalar: f32) -> Quaternion {
        Quaternion {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
            w: self.w / scalar,
        }
    }

    pub fn from_euler_angles(pitch: f32, yaw: f32, roll: f32) -> Self {
        let hp = pitch * 0.5;
        let hy = yaw * 0.5;
        let hr = roll * 0.5;
        let (cp, sp) = (hp.cos(), hp.sin());
        let (cy, sy) = (hy.cos(), hy.sin());
        let (cr, sr) = (hr.cos(), hr.sin());

        Quaternion {
            x: sp * cy * cr - cp * sy * sr,
            y: cp * sy * cr + sp * cy * sr,
            z: cp * cy * sr - sp * sy * cr,
            w: cp * cy * cr + sp * sy * sr,
        }
    }

    pub fn from_axis_angle(axis: Vec3, angle: f32) -> Self {
        let (s, c) = (angle * 0.5).sin_cos();
        Quaternion {
            x: axis.x * s,
            y: axis.y * s,
            z: axis.z * s,
            w: c,
        }
    }

    pub fn normalize(&self) -> Self {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt();
        if len > 0.0 {
            self.div(len)
        } else {
            Quaternion::identity()
        }
    }

    pub fn inverse(&self) -> Self {
        Quaternion {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w,
        }
        .normalize()
    }

    pub fn to_rotation_matrix(&self) -> Mat4 {
        let x2 = self.x + self.x;
        let y2 = self.y + self.y;
        let z2 = self.z + self.z;

        let xx = self.x * x2;
        let xy = self.x * y2;
        let xz = self.x * z2;

        let yy = self.y * y2;
        let yz = self.y * z2;
        let zz = self.z * z2;

        let wx = self.w * x2;
        let wy = self.w * y2;
        let wz = self.w * z2;

        Mat4 {
            data: [
                [1.0 - (yy + zz), xy - wz, xz + wy, 0.0],
                [xy + wz, 1.0 - (xx + zz), yz - wx, 0.0],
                [xz - wy, yz + wx, 1.0 - (xx + yy), 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn rotate_vector(&self, v: Vec3) -> Vec3 {
        let q_vec = Vec3::new(self.x, self.y, self.z);
        // standard formula: v' = v + 2 * (q_vec x (q_vec x v) + w * (q_vec x v))
        let t = q_vec.cross(&v).mul_scalar(2.0);
        v.add(&t.mul_scalar(self.w)).add(&q_vec.cross(&t))
    }

    pub fn slerp(&self, other: &Quaternion, t: f32) -> Quaternion {
        let mut cos_half_theta = self.dot(other);
        if cos_half_theta < 0.0 {
            cos_half_theta = -cos_half_theta;
        }

        if cos_half_theta > 0.9995 {
            return self.mul_f32(1.0 - t).add(&other.mul_f32(t)).normalize();
        }

        let half_theta = cos_half_theta.acos();
        let sin_half_theta = (1.0 - cos_half_theta * cos_half_theta).sqrt();

        if sin_half_theta.abs() < 0.001 {
            return self.mul_f32(0.5).add(&other.mul_f32(0.5)).normalize();
        }

        let ratio_a = ((1.0 - t) * half_theta).sin() / sin_half_theta;
        let ratio_b = (t * half_theta).sin() / sin_half_theta;

        self.mul_f32(ratio_a)
            .add(&other.mul_f32(ratio_b))
            .normalize()
    }

    pub fn lerp(&self, other: &Quaternion, t: f32) -> Quaternion {
        self.mul_f32(1.0 - t).add(&other.mul_f32(t)).normalize()
    }

    pub fn rotate_pitch(&mut self, angle: f32) {
        // FPS-style pitch: rotate around camera's local right axis and clamp pitch
        let axis = self.rotate_vector(RIGHT);
        let q = Quaternion::from_axis_angle(axis, angle);
        let new_q = q.mul(self).normalize();

        // Prevent flipping: compute pitch from forward vector's y component
        let fwd_y = new_q.rotate_vector(FORWARD).y.clamp(-1.0, 1.0);
        let pitch_angle = fwd_y.asin();
        let limit = std::f32::consts::FRAC_PI_2 - 0.01; // ~89.4 degrees
        if pitch_angle.abs() < limit {
            *self = new_q;
        }
    }

    pub fn get_pitch(&self) -> f32 {
        let fwd_y = self.rotate_vector(FORWARD).y.clamp(-1.0, 1.0);
        fwd_y.asin()
    }

    pub fn rotate_yaw(&mut self, angle: f32) {
        // Yaw around global up (world Y)
        // Use negative angle so positive input rotates camera to the right (FPS convention)
        let q = Quaternion::from_axis_angle(UP, -angle);
        *self = q.mul(self).normalize();
    }

    pub fn get_yaw(&self) -> f32 {
        let fwd = self.rotate_vector(FORWARD);
        fwd.z.atan2(fwd.x)
    }

    pub fn rotate_roll(&mut self, angle: f32) {
        let q = Quaternion::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), angle);
        *self = q.mul(self).normalize();
    }

    pub fn get_roll(&self) -> f32 {
        let right = self.rotate_vector(RIGHT);
        right.y.atan2(right.x)
    }

    pub fn rotate_around_axis(&mut self, axis: Vec3, angle: f32) {
        let q = Quaternion::from_axis_angle(axis, angle);
        *self = q.mul(self).normalize();
    }

    pub fn forward(&self) -> Vec3 {
        self.rotate_vector(FORWARD)
    }

    pub fn right(&self) -> Vec3 {
        self.rotate_vector(RIGHT)
    }

    pub fn up(&self) -> Vec3 {
        self.rotate_vector(UP)
    }
}
