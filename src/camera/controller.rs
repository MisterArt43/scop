use glfw::Key;

use crate::{
    app::event::InputState,
    camera::{camera::Camera, view::ViewMode},
    math::{quaternion::Quaternion, vec3::Vec3},
};

pub struct CameraController {
    pub move_speed: f32,
    pub mouse_sensitivity: f32,

    yaw: f32,
    pitch: f32,

    last_mouse_position: Option<(f64, f64)>,
}

impl CameraController {
    pub fn new(move_speed: f32, mouse_sensitivity: f32) -> Self {
        Self {
            move_speed,
            mouse_sensitivity,

            yaw: 0.0,
            pitch: 0.0,

            last_mouse_position: None,
        }
    }

    pub fn from_camera(camera: &Camera, move_speed: f32, mouse_sensitivity: f32) -> Self {
        let forward = camera.forward();

        let pitch = forward.y.asin();

        let yaw = (-forward.x).atan2(-forward.z);

        Self {
            move_speed,
            mouse_sensitivity,

            yaw,
            pitch,

            last_mouse_position: None,
        }
    }

    pub fn update(&mut self, camera: &mut Camera, input: &InputState, delta_time: f32) {
        if !matches!(camera.view_mode, ViewMode::Free) {
            return;
        }

        self.update_movement(camera, input, delta_time);

        self.update_rotation(camera, input);
    }

    fn update_movement(&self, camera: &mut Camera, input: &InputState, delta_time: f32) {
        let basis = camera.basis();

        let mut direction = Vec3::new(0.0, 0.0, 0.0);

        if input.key_down(Key::W) {
            direction += basis.forward;
        }

        if input.key_down(Key::S) {
            direction -= basis.forward;
        }

        if input.key_down(Key::D) {
            direction += basis.right;
        }

        if input.key_down(Key::A) {
            direction -= basis.right;
        }

        if input.key_down(Key::Space) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if input.key_down(Key::LeftControl) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();

            camera.transform.position += direction * self.move_speed * delta_time;
        }
    }

    fn update_rotation(&mut self, camera: &mut Camera, input: &InputState) {
        let Some((x, y)) = input.mouse_position else {
            self.last_mouse_position = None;
            return;
        };

        let Some((last_x, last_y)) =
            self.last_mouse_position
        else {
            self.last_mouse_position = Some((x, y));
            return;
        };

        let delta_x = (x - last_x) as f32;
        let delta_y = (y - last_y) as f32;

        self.last_mouse_position = Some((x, y));

        self.yaw -= delta_x * self.mouse_sensitivity;

        self.pitch -= delta_y * self.mouse_sensitivity;

        let max_pitch = 89.0_f32.to_radians();

        self.pitch = self.pitch.clamp(-max_pitch, max_pitch);

        let yaw_rotation = Quaternion::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), self.yaw);

        let pitch_rotation = Quaternion::from_axis_angle(Vec3::new(1.0, 0.0, 0.0), self.pitch);

        camera.transform.rotation = yaw_rotation.mul(&pitch_rotation).normalize();
    }

    pub fn reset_mouse(&mut self) {
        self.last_mouse_position = None;
    }

    pub fn set_speed(&mut self, speed: f32) {
        self.move_speed = speed;
    }

    pub fn set_sensitivity(&mut self, sensitivity: f32) {
        self.mouse_sensitivity = sensitivity;
    }
}
