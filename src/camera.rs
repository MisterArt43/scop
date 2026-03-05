use glfw::ffi::glfwGetTime;

use crate::math::{mat4::Mat4, vec3::Vec3};


// ignore unused for now
#[allow(unused)]
pub struct Camera {
    position: Vec3,
    orientation: Vec3,
    field_of_view: f32,
    pub move_speed: f32,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            position: Vec3::new(0.0, 0.0, 3.0),
            orientation: Vec3::new(0.0, 0.0, 0.0),
            field_of_view: 90.0,
            move_speed: 1.0,
        }
    }

    pub fn update_basic_rot(&mut self) {
        let camera_target = Vec3::new(0.0, 0.0, 0.0);
        let camera_direction = self.position.sub(&camera_target).normalize();

        let up = Vec3::new(0.0, 1.0, 0.0);
        let camera_right = up.cross(&camera_direction).normalize();

        let camera_up = camera_direction.cross(&camera_right);

        
        const radius: f32 = 10.0;
        let mut cam_x:f32 = 0.0;
        let mut cam_z:f32 = 0.0;
        
        unsafe {
            cam_x = radius * glfwGetTime().sin() as f32;
            cam_z = radius * glfwGetTime().cos() as f32;
        }
        
        let view: Mat4 = Mat4::look_at(Vec3 { x: cam_x, y: 0.0, z: cam_z }, camera_target, camera_up);

        println!("Camera position: {:?}", self.position);
        println!("Camera orientation: {:?}", self.orientation);
        println!("Camera view matrix: {:?}", view);

        // apply the view matrix to the viewport
    }

    // pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
    //     self.field_of_view = aspect_ratio;
    // }
}