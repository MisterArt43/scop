use std::{collections::HashSet, time::{Duration, Instant}};

use glfw::{Context, WindowHint::ContextVersion, ffi::glfwTerminate};

use crate::app::app_event;


pub struct Application {
    glfw: glfw::Glfw,
    pub(crate) window: glfw::PWindow,
    events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
    pub event_handler: app_event::AppEvent,
    pressed_keys: HashSet<glfw::Key>,
    pub width: i32,
    pub height: i32,

    last_time: Instant,

    delta_time: f32,
    pub last_frame_time: f32,
    pub to_rerender: bool,

    pub curpos: (f32, f32),
}

impl Application {
    pub fn terminate(&mut self) {
        unsafe {
            glfwTerminate();
        }
    }

    pub fn new(name: &str, width: f32, height: f32) -> Application {
        // initialisation de GLFW
        let mut glfw = glfw::init(error_callback).expect("Failed to initialize GLFW");

        // Set des param de fenetre
        glfw.window_hint(ContextVersion(3, 3)); //version opengl
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        glfw.window_hint(glfw::WindowHint::DepthBits(Some(24))); // le depth buffer sert a stocker la profondeur de chaque pixel on appelle ca le z-buffer et il est de 24 bits, (plus il est grand plus on peut stocker de profondeur et moins on a de probleme de z-fighting(texture qui clip entre elle)) // mais attention car un buffer trop grand peut aussi causer des problemes de performance
        glfw.window_hint(glfw::WindowHint::Samples(Some(4)));
        // to use GPU rendering instead ofCPU integrated rendering (if available)
        glfw.window_hint(glfw::WindowHint::DoubleBuffer(true));
        glfw.window_hint(glfw::WindowHint::Resizable(true));
        glfw.window_hint(glfw::WindowHint::Visible(true));

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
            glfw,
            window,
            events,
            event_handler: app_event::AppEvent::new(),
            width: 0,
            height: 0,
            pressed_keys: HashSet::new(),
            delta_time: 0.0,
            last_time: Instant::now(),
            to_rerender: true,
            curpos: (0.0, 0.0),
        };
        app.window.set_key_polling(true);
        app.window.set_cursor_pos_polling(true);
        app.window.make_current();
        // swap les commentaire pour activer ou desactiver le vsync
        app.glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
        // app.glfw.set_swap_interval(glfw::SwapInterval::None);
        app.window.set_cursor_mode(glfw::CursorMode::Disabled);
        app.window
            .set_cursor_pos(f64::from(width) / 2.0, f64::from(height) / 2.0);
        app.window.request_attention();

        app.my_init_gl();
        app
    }

    fn my_init_gl(&mut self) {
        gl_loader::init_gl();
        gl::load_with(|s| {
            self.window
                .get_proc_address(s)
                .map_or(std::ptr::null(), |f| f as *const _)
        });

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            gl::ClearDepth(1.0);
            gl::Disable(gl::CULL_FACE);
            gl::Enable(gl::MULTISAMPLE);
        }
    }

    pub fn update_delta_time(&mut self) {
        let current_frame_time = self.glfw.get_time() as f32;
        self.delta_time = current_frame_time - self.last_frame_time;
        self.last_frame_time = current_frame_time;

        let elapsed = self.last_time.elapsed();

        if elapsed >= Duration::from_millis(300) {
            self.window
                .set_title(&format!("Scop - {:.0}", 1.0 / self.delta_time.max(0.00001)));

            self.last_time = Instant::now();
        }
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
            // println!("Event: {:?}", event);
            match event {
                glfw::WindowEvent::Close => self.window.set_should_close(true),
                glfw::WindowEvent::Key(key, _scancode, action, _mods) => {
                    if action == glfw::Action::Press {
                        self.pressed_keys.insert(key);
                    } else if action == glfw::Action::Release {
                        self.pressed_keys.remove(&key);
                    }

                    if key == glfw::Key::LeftControl {
                        unsafe {
                            if action == glfw::Action::Release {
                                gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
                            } else if action == glfw::Action::Press {
                                gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
                            }
                        }
                    }
                    if key == glfw::Key::LeftAlt {
                        if action == glfw::Action::Release {
                            //Disable mouse capture
                            self.window.set_cursor_mode(glfw::CursorMode::Disabled);
                        } else if action == glfw::Action::Press {
                            //Enable mouse capture
                            self.window.set_cursor_mode(glfw::CursorMode::Normal);
                        }
                    }
                    if key == glfw::Key::LeftShift {
                        unsafe {
                            if action == glfw::Action::Release {
                                gl::Enable(gl::CULL_FACE);
                            } else if action == glfw::Action::Press {
                                gl::Disable(gl::CULL_FACE);
                            }
                        }
                    }

                    if key == glfw::Key::C && action == glfw::Action::Press {
                        self.to_rerender = true;
                        // self.camera.mode = (self.camera.mode + 1) % 2;
                        // println!(
                        //     "Camera mode: {}",
                        //     if self.camera.mode == 0 {
                        //         "Free"
                        //     } else {
                        //         "Look-At"
                        //     }
                        // );
                    }

                    if key == glfw::Key::Escape && action == glfw::Action::Press {
                        self.window.set_should_close(true);
                    }
                }
                glfw::WindowEvent::CursorPos(xpos, ypos) => {
                    let xoffset = self.curpos.0 - xpos as f32;
                    let yoffset = ypos as f32 - self.curpos.1; // reversed since y-coordinates go from bottom to top
                    let sensitivity = 0.5; // change this value to your liking
                    let _xoffset = xoffset * sensitivity;
                    let _yoffset = yoffset * sensitivity;

                    if self.window.get_cursor_mode() == glfw::CursorMode::Disabled {
                        // self.camera.process_mouse_movement(
                        //     xoffset * self.delta_time,
                        //     yoffset * self.delta_time,
                        // );
                        // self.to_rerender = true;
                    }

                    self.curpos = (xpos as f32, ypos as f32);
                }
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    self.width = width;
                    self.height = height;
                    unsafe {
                        gl::Viewport(0, 0, width, height);
                    }
                }
                _ => {}
            }
        }
        self.update();
    }

    pub fn update(&mut self) {
        
    }
}

fn error_callback(err: glfw::Error, description: String) {
    eprintln!("GLFW error {:?}: {:?}", err, description);
}