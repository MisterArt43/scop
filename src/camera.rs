use crate::{
    math::{
        mat4::Mat4,
        transform::Transform,
        vec3::{FORWARD, RIGHT, UP, Vec3},
    },
    mesh::SubMesh,
};

// ignore unused for now
#[allow(unused)]
pub struct Camera {
    pub transform: Transform,
    field_of_view: f32,
    pub move_speed: f32,
    pub model: Mat4,
    pub view: Mat4,
    pub projection: Mat4,
    pub camera_distance: f32,
    pub mode: u8,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            transform: Transform::new(),
            field_of_view: 60.0,
            move_speed: 1.0,
            model: Mat4::identity(),
            view: Mat4::new(),
            projection: Mat4::new(),
            camera_distance: 10.0,
            mode: 0,
        }
    }

    pub fn init_view(&mut self, sub_mesh: &Vec<SubMesh>) {
        let mut scene_min = [f32::INFINITY; 3];
        let mut scene_max = [f32::NEG_INFINITY; 3];
        for sub in sub_mesh {
            for i in 0..3 {
                if sub.mesh.bbox_min[i] < scene_min[i] {
                    scene_min[i] = sub.mesh.bbox_min[i];
                }
                if sub.mesh.bbox_max[i] > scene_max[i] {
                    scene_max[i] = sub.mesh.bbox_max[i];
                }
            }
        }
        let center = Vec3::new(
            (scene_min[0] + scene_max[0]) * 0.5,
            (scene_min[1] + scene_max[1]) * 0.5,
            (scene_min[2] + scene_max[2]) * 0.5,
        );
        let diag = (scene_max[0] - scene_min[0])
            .max(scene_max[1] - scene_min[1])
            .max(scene_max[2] - scene_min[2]);

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

    pub fn move_forward(&mut self, distance: f32) {
        let forward = self.transform.rotation.rotate_vector(FORWARD);
        self.transform.position += forward.mul_scalar(distance);
    }

    pub fn move_right(&mut self, distance: f32) {
        let right = self.transform.rotation.rotate_vector(RIGHT);
        self.transform.position = self.transform.position.add(&right.mul_scalar(distance));
    }

    pub fn move_up(&mut self, distance: f32) {
        let up = self.transform.rotation.rotate_vector(UP);
        self.transform.position = self.transform.position.add(&up.mul_scalar(distance));
    }

    pub fn update_view_and_projection(&mut self, fb_width: f32, fb_height: f32) {
        let aspec = fb_width / fb_height;
        self.projection = Mat4::perspective(self.field_of_view, aspec, 0.001, 500.0);

        ///////////////////
        //  FREE CAMERA ///
        ///////////////////
        if self.mode == 0 {
            let eye = self.transform.position;
            let target = self
                .transform
                .position
                .add(&self.transform.rotation.forward());
            let up = self.transform.rotation.rotate_vector(UP);
            self.view = Mat4::look_at(eye, target, up);
        }
        ///////////////////
        //  FREE CAMERA ///
        ///////////////////

        /////////////////////
        // LOOK-AT CAMERA /// -> orbite autour de la target
        /////////////////////

        if self.mode == 1 {
            // Compute orbiting eye position from quaternion forward vector
            let target = Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            let forward = self.transform.rotation.forward();
            let eye = target.sub(&forward.mul_scalar(self.camera_distance));
            // Use world up for stable orbiting
            let up = Vec3::new(0.0, 1.0, 0.0);
            self.view = Mat4::look_at(eye, target, up);
        }
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.field_of_view = aspect_ratio;
    }
}
