use glfw::ffi::glfwGetTime;

use crate::{camera, math::{mat4::Mat4, vec3::Vec3}, mesh::SubMesh};


// ignore unused for now
#[allow(unused)]
pub struct Camera {
    position: Vec3,
    pub orientation: Vec3,
    field_of_view: f32,
    pub move_speed: f32,
    pub model: Mat4,
    pub view: Mat4,
    pub projection: Mat4,
    pub camera_distance: f32,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            position: Vec3::new(0.0, 0.0, 3.0),
            orientation: Vec3::new(0.0, 0.0, 0.0),
            field_of_view: 90.0,
            move_speed: 1.0,
            model: Mat4::identity(),
            view: Mat4::new(),
            projection: Mat4::new(),
            camera_distance: 10.0,
        }
    }

    pub fn init_view(&mut self, sub_mesh: &Vec<SubMesh>) {
        let mut scene_min = [f32::INFINITY; 3];
        let mut scene_max = [f32::NEG_INFINITY; 3];
        for sub in sub_mesh {
            for i in 0..3 {
                if sub.mesh.bbox_min[i] < scene_min[i] { scene_min[i] = sub.mesh.bbox_min[i]; }
                if sub.mesh.bbox_max[i] > scene_max[i] { scene_max[i] = sub.mesh.bbox_max[i]; }
            }
        }
        let center = Vec3::new(
            (scene_min[0] + scene_max[0]) * 0.5,
            (scene_min[1] + scene_max[1]) * 0.5,
            (scene_min[2] + scene_max[2]) * 0.5,
        );
        let diag = (scene_max[0] - scene_min[0]).max(scene_max[1] - scene_min[1]).max(scene_max[2] - scene_min[2]);

        let scale_factor = 1.0 / diag.max(1.0);
        self.model.data[0][0] = scale_factor;
        self.model.data[1][1] = scale_factor;
        self.model.data[2][2] = scale_factor;
        self.model.data[3][0] = -center.x * scale_factor;
        self.model.data[3][1] = -center.y * scale_factor;
        self.model.data[3][2] = -center.z * scale_factor;

        let camera_distance = diag * scale_factor / 1.3;
        self.camera_distance = camera_distance;
    }

    pub fn rotate_pitch(&mut self, angle: f32) {
        self.orientation.x += angle;
    }

    pub fn rotate_yaw(&mut self, angle: f32) {
        self.orientation.y += angle;
    }

    pub fn rotate_roll(&mut self, angle: f32) {
        self.orientation.z += angle;
    }

    pub fn move_forward(&mut self, distance: f32) {
        let forward = Vec3 {
            x: self.orientation.y.cos() * self.orientation.x.cos(),
            y: self.orientation.x.sin(),
            z: self.orientation.y.sin() * self.orientation.x.cos(),
        };
        self.position = self.position.add(&forward.mul_scalar(distance));
    }

    pub fn move_right(&mut self, distance: f32) {
        let right = Vec3 {
            x: self.orientation.y.cos() * (self.orientation.x + std::f32::consts::FRAC_PI_2).cos(),
            y: (self.orientation.x + std::f32::consts::FRAC_PI_2).sin(),
            z: self.orientation.y.sin() * (self.orientation.x + std::f32::consts::FRAC_PI_2).cos(),
        };
        self.position = self.position.add(&right.mul_scalar(distance));
    }

    pub fn update_view_and_projection(&mut self, fb_width: f32, fb_height: f32) {
        let aspec = fb_width / fb_height;
        self.projection = Mat4::perspective(self.field_of_view, aspec, 0.1, 500.0);;

        // let angle = 0.0 as f32;//unsafe { glfwGetTime() as f32 }; //temp
        // let angle = unsafe { glfwGetTime() as f32 }; //temp
        // let eye_x = angle.cos() * self.camera_distance; //temp
        // let eye_z = angle.sin() * self.camera_distance; //temp
        // let eye = Vec3 { x: eye_x, y: 0.0, z: eye_z }; //temp

        let eye = Vec3 {
            x: self.camera_distance * self.orientation.y.cos() * self.orientation.x.cos(),
            y: self.camera_distance * self.orientation.x.sin(),
            z: self.camera_distance * self.orientation.y.sin() * self.orientation.x.cos(),
        };


        self.view = Mat4::look_at(eye, Vec3 { x: 0.0, y: 0.0, z: 0.0 }, Vec3::new(0.0, 1.0, 0.0));
         
        
        // let camera_target = Vec3::new(0.0, 0.0, 0.0);
        // let camera_direction = self.position.sub(&camera_target).normalize();

        // let up = Vec3::new(0.0, 1.0, 0.0);
        // let camera_right = up.cross(&camera_direction).normalize();

        // let camera_up = camera_direction.cross(&camera_right);

        // const RADIUS: f32 = 10.0;
        // let cam_x:f32;
        // let cam_z:f32;

        // unsafe {
        //     cam_x = RADIUS * glfwGetTime().sin() as f32;
        //     cam_z = RADIUS * glfwGetTime().cos() as f32;
        // }

        // self.view = Mat4::look_at(Vec3 { x: cam_x, y: 0.0, z: cam_z }, camera_target, camera_up);
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.field_of_view = aspect_ratio;
    }
}