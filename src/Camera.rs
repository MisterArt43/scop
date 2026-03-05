use crate::math::vec3::Vec3;



pub struct Camera {
    position: Vec3,
    orientation: Vec3,
    field_of_view: f32,
    pub move_speed: f32,
}

impl Camera {
    pub fn new() -> Self {
        let mut cam =Camera {
            position: Vec3::new(0.0, 0.0, 3.0),
            orientation: Vec3::new(0.0, 0.0, 0.0),
            field_of_view: 90.0,
            move_speed: 1.0,
        };

        let camera_target = Vec3::new(0.0, 0.0, 0.0);
        let camera_direction = cam.position.sub(&camera_target).normalize();

        let up = Vec3::new(0.0, 1.0, 0.0);
        let camera_right = up.cross(&camera_direction).normalize();

        let camera_up = camera_direction.cross(&camera_right);

        cam
    }

    pub fn updateAxesFromAngles(&mut self) {
        let fornt (yaw, pitch, roll) = self.orientation;
        let front_x = yaw.cos() * pitch.cos();
        let front_y = pitch.sin();
        let front_z = yaw.sin() * pitch.cos();
    }
}