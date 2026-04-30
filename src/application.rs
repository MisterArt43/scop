use gl::{ClearDepth, DEPTH_TEST, DepthFunc, Disable, Enable};
use gl_loader::init_gl;
use glfw::{Context, Glfw, GlfwReceiver, PWindow, WindowEvent, WindowHint};

use crate::camera::Camera;

// ignore unused for now
#[allow(unused)]
pub struct Application {
    glfw: Glfw,
    pub(crate) window: PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,

    width: f32,
    height: f32,

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

        let (window, events) = glfw
            .create_window(
                width as u32,
                height as u32,
                name,
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create windows");
        let mut app = Application {
            glfw: glfw,
            window: window,
            events: events,
            width,
            height,
            name: String::from(name),
            curpos: (0.0, 0.0),
            camera: Camera::new(),
        };
        app.window.set_key_polling(true);
        app.window.make_current();

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
    }
}

fn error_callback(err: glfw::Error, description: String) {
    eprintln!("GLFW error {:?}: {:?}", err, description);
}
