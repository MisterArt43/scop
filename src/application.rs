use gl::{CULL_FACE, ClearDepth, DEPTH_TEST, DepthFunc, Disable, Enable};
use gl_loader::init_gl;
use glfw::{Context, Glfw, GlfwReceiver, PWindow, WindowEvent, WindowHint};
use std::collections::HashSet;

use crate::camera::Camera;

#[allow(unused)]
pub struct Application {
    glfw: Glfw,
    pub(crate) window: PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,
    pressed_keys: HashSet<glfw::Key>,
    width: f32,
    height: f32,
    delta_time: f32,
    pub last_frame_time: f32,

    name: String,
    pub curpos: (f32, f32),
    pub(crate) camera: Camera,
}

impl Application {
    pub fn new(name: &str, width: f32, height: f32) -> Application {
        // initialisation de GLFW
        let mut glfw = glfw::init(error_callback).expect("Failed to initialize GLFW");

        // Set des param de fenetre
        WindowHint::ContextVersion(3, 3); //version opengl
        WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core);
        WindowHint::DepthBits(Some(24)); // le depth buffer sert a stocker la profondeur de chaque pixel on appelle ca le z-buffer et il est de 24 bits, (plus il est grand plus on peut stocker de profondeur et moins on a de probleme de z-fighting(texture qui clip entre elle)) // mais attention car un buffer trop grand peut aussi causer des problemes de performance
        WindowHint::Samples(Some(4));
        // to use GPU rendering instead ofCPU integrated rendering (if available)
        WindowHint::DoubleBuffer(true);
        WindowHint::Resizable(true);
        WindowHint::Visible(true);

        let (window, events) = glfw
            .create_window(
                width as u32,
                height as u32,
                name,
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create windows");
        let mut app = Application {
            last_frame_time: glfw.get_time() as f32,
            glfw: glfw,
            window: window,
            events: events,
            width,
            height,
            delta_time: 0.0,
            name: String::from(name),
            curpos: (0.0, 0.0),
            camera: Camera::new(),
            pressed_keys: HashSet::new(),
        };
        app.window.set_key_polling(true);
        app.window.make_current();
        app.glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
        

        (&mut app).my_init_gl();
        app
    }

    fn my_init_gl(&mut self) {
        init_gl();
        gl::load_with(|s| {
            self.window
                .get_proc_address(s)
                .map_or(std::ptr::null(), |f| f as *const _)
        });

        unsafe {
            Enable(DEPTH_TEST);
            DepthFunc(gl::LESS);
            ClearDepth(1.0);
            Disable(gl::CULL_FACE);
            Enable(gl::MULTISAMPLE);
        }
    }

    pub fn update_delta_time(&mut self) {
        let current_frame_time = self.glfw.get_time() as f32;
        self.delta_time = current_frame_time - self.last_frame_time;
        self.last_frame_time = current_frame_time;
    }

    pub fn deltatime(&self) -> f32 {
        self.delta_time
    }

    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers();
    }

    pub fn handle_events(&mut self) {
        // iterate over all pending events
        // poll GLFW to populate the event queue, then iterate over all pending events
        self.glfw.poll_events();
        for (_id, event) in glfw::flush_messages(&self.events) {
            println!("Event: {:?}", event);
            match event {
                WindowEvent::Close => self.window.set_should_close(true),
                WindowEvent::Key(key, _scancode, action, _mods) => {
                    if action == glfw::Action::Press {
                        self.pressed_keys.insert(key);
                    } else if action == glfw::Action::Release {
                        self.pressed_keys.remove(&key);
                    }
                    if key == glfw::Key::LeftAlt {
                        unsafe {
                            if action == glfw::Action::Release {
                                gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
                            } else if action == glfw::Action::Press {
                                gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
                            }
                        }
                    }
                    if key == glfw::Key::LeftShift {
                        unsafe {
                            if action == glfw::Action::Release {
                                gl::Enable(CULL_FACE);
                            } else if action == glfw::Action::Press {
                                gl::Disable(CULL_FACE);
                            }
                        }
                    }

                    if key == glfw::Key::C && action == glfw::Action::Press {
                        self.camera.mode = (self.camera.mode + 1) % 2;
                        println!(
                            "Camera mode: {}",
                            if self.camera.mode == 0 {
                                "Free"
                            } else {
                                "Look-At"
                            }
                        );
                    }

                    if key == glfw::Key::Escape && action == glfw::Action::Press {
                        self.window.set_should_close(true);
                    }
                }
                WindowEvent::CursorPos(xpos, ypos) => {
                    self.curpos = (xpos as f32, ypos as f32);
                }
                _ => {}
            }
        }
        self.update();
    }

    pub fn update(&mut self) {
        // pr gerer les inputs pr la cam
        let zoom_speed = 1.2 * self.delta_time; // Adjust as needed
        let move_speed = 1.0 * self.delta_time * self.camera.camera_distance; // Adjust as needed
        let rotation_speed = 2.0 * self.delta_time * self.camera.camera_distance; // Adjust as needed
        if self.pressed_keys.contains(&glfw::Key::W) {
            self.camera.move_forward(move_speed);
        }
        if self.pressed_keys.contains(&glfw::Key::S) {
            self.camera.move_forward(-move_speed);
        }
        if self.pressed_keys.contains(&glfw::Key::A) {
            self.camera.move_right(-move_speed);
        }
        if self.pressed_keys.contains(&glfw::Key::D) {
            self.camera.move_right(move_speed);
        }
        if self.pressed_keys.contains(&glfw::Key::Q) {
            self.camera.move_up(-move_speed);
        }
        if self.pressed_keys.contains(&glfw::Key::E) {
            self.camera.move_up(move_speed);
        }
        if self.pressed_keys.contains(&glfw::Key::Up) {
            self.camera.transform.rotation.rotate_pitch(rotation_speed);
        }
        if self.pressed_keys.contains(&glfw::Key::Down) {
            self.camera.transform.rotation.rotate_pitch(-rotation_speed);
        }
        if self.pressed_keys.contains(&glfw::Key::Left) {
            self.camera.transform.rotation.rotate_yaw(-rotation_speed);
        }
        if self.pressed_keys.contains(&glfw::Key::Right) {
            self.camera.transform.rotation.rotate_yaw(rotation_speed);
        }
        if self.pressed_keys.contains(&glfw::Key::KpAdd) {
            self.camera.camera_distance -= zoom_speed;
            if self.camera.camera_distance < 0.1 {
                self.camera.camera_distance = 0.1;
            }
        }
        if self.pressed_keys.contains(&glfw::Key::KpSubtract) {
            self.camera.camera_distance += zoom_speed;
            if self.camera.camera_distance > 100.0 {
                self.camera.camera_distance = 100.0;
            }
        }
        // if self.pressed_keys.contains(&glfw::Key::C) { self.camera.mode = (self.camera.mode + 1) % 2; println!("Camera mode: {}", if self.camera.mode == 0 { "Free" } else { "Look-At" }); }
    }
}

fn error_callback(err: glfw::Error, description: String) {
    eprintln!("GLFW error {:?}: {:?}", err, description);
}
