use std::{collections::HashSet, path::PathBuf};

use glfw::PWindow;

pub struct AppEvent {
    pub input: InputState,
    pub frame_buffer_size: (i32, i32),
    pub focus: bool,
    pub iconified: bool, // Indique si la fenêtre est minimize
    pub cursor_entered: bool,
    pub file_dropped: Option<Vec<PathBuf>>,
    pub maximized: bool,
    should_close: bool,
}

pub struct InputState {
    pub key_down: HashSet<glfw::Key>,
    pub mouse_button_down: HashSet<glfw::MouseButton>,

    pub key_pressed: HashSet<glfw::Key>,
    pub key_released: HashSet<glfw::Key>,

    pub mouse_position: Option<(f64, f64)>, // curpos
    pub mouse_scrolled: Option<(f64, f64)>,

    pub mouse_button_pressed: HashSet<glfw::MouseButton>,
    pub mouse_button_released: HashSet<glfw::MouseButton>,

    pub modifiers: glfw::Modifiers,
}

impl InputState {
    fn new() -> Self {
        InputState {
            key_down: HashSet::new(),
            mouse_button_down: HashSet::new(),

            key_pressed: HashSet::new(),
            key_released: HashSet::new(),

            mouse_position: None,
            mouse_scrolled: None,

            mouse_button_pressed: HashSet::new(),
            mouse_button_released: HashSet::new(),

            modifiers: glfw::Modifiers::empty(),
        }
    }

    pub fn shift(&self) -> bool {
        self.modifiers.contains(glfw::Modifiers::Shift)
    }

    pub fn control(&self) -> bool {
        self.modifiers.contains(glfw::Modifiers::Control)
    }

    pub fn alt(&self) -> bool {
        self.modifiers.contains(glfw::Modifiers::Alt)
    }

    pub fn super_key(&self) -> bool {
        self.modifiers.contains(glfw::Modifiers::Super)
    }

    pub fn key_down(&self, key: glfw::Key) -> bool {
        self.key_down.contains(&key)
    }

    pub fn key_pressed(&self, key: glfw::Key) -> bool {
        self.key_pressed.contains(&key)
    }

    pub fn key_released(&self, key: glfw::Key) -> bool {
        self.key_released.contains(&key)
    }

    pub fn mouse_down(&self, button: glfw::MouseButton) -> bool {
        self.mouse_button_down.contains(&button)
    }

    pub fn mouse_pressed(&self, button: glfw::MouseButton) -> bool {
        self.mouse_button_pressed.contains(&button)
    }

    pub fn mouse_released(&self, button: glfw::MouseButton) -> bool {
        self.mouse_button_released.contains(&button)
    }
}

impl AppEvent {
    pub fn new() -> Self {
        AppEvent {
            input: InputState::new(),
            frame_buffer_size: (0, 0),
            focus: false,
            iconified: false,
            cursor_entered: false,
            file_dropped: None,
            maximized: false,
            should_close: false,
        }
    }

    pub fn init_glfw_events(&mut self, window: &mut PWindow) {
        window.set_close_polling(true);
        window.set_focus_polling(true);
        window.set_iconify_polling(true);
        window.set_framebuffer_size_polling(true);
        window.set_mouse_button_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_cursor_enter_polling(true);
        window.set_scroll_polling(true);
        window.set_key_polling(true);
        window.set_drag_and_drop_polling(true);
        window.set_maximize_polling(true);
    }

    pub fn handle_event(
        &mut self,
        glfw: &mut glfw::Glfw,
        window: &mut PWindow,
        events: &glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
    ) {
        glfw.poll_events();
        for (_id, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Close => {
                    self.close_event(window);
                }
                glfw::WindowEvent::Focus(focused) => {
                    self.focus_event(focused);
                }
                glfw::WindowEvent::Iconify(iconified) => {
                    self.iconify_event(iconified);
                }
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    self.framebuffer_size_event(width, height);
                }
                glfw::WindowEvent::MouseButton(button, action, mods) => {
                    self.mouse_button_event(button, action, mods);
                }
                glfw::WindowEvent::CursorPos(x, y) => {
                    self.mouse_move_event(x, y);
                }
                glfw::WindowEvent::CursorEnter(enter) => {
                    self.cursor_enter_event(enter);
                }
                glfw::WindowEvent::Scroll(x, y) => {
                    self.mouse_scroll_event(x, y);
                }
                glfw::WindowEvent::Key(key, _scancode, action, mods) => {
                    self.key_event(key, action, mods);
                }
                glfw::WindowEvent::FileDrop(paths) => {
                    self.file_drop_event(paths);
                }
                glfw::WindowEvent::Maximize(maximized) => {
                    self.maximize_event(maximized);
                }
                _ => {}
            }
        }
        if self.should_close {
            self.close_event(window);
        }
    }

    fn key_event(&mut self, key: glfw::Key, action: glfw::Action, mods: glfw::Modifiers) {
        if key == glfw::Key::Escape && action == glfw::Action::Press {
            self.should_close = true;
        }
        self.input.modifiers = mods; // Met à jour le masque de modificatrices

        match action {
            glfw::Action::Press => {
                self.input.key_down.insert(key);
                self.input.key_pressed.insert(key);
                self.input.key_released.remove(&key);
            }
            glfw::Action::Release => {
                self.input.key_down.remove(&key);
                self.input.key_released.insert(key);
                self.input.key_pressed.remove(&key);
            }
            glfw::Action::Repeat => {}
        }
    }

    fn mouse_button_event(
        &mut self,
        button: glfw::MouseButton,
        action: glfw::Action,
        mods: glfw::Modifiers,
    ) {
        self.input.modifiers = mods; // Met également à jour sur les clics de souris

        match action {
            glfw::Action::Press => {
                self.input.mouse_button_down.insert(button);

                self.input.mouse_button_pressed.insert(button);
                self.input.mouse_button_released.remove(&button);
            }
            glfw::Action::Release => {
                self.input.mouse_button_down.remove(&button);

                self.input.mouse_button_released.insert(button);
                self.input.mouse_button_pressed.remove(&button);
            }
            glfw::Action::Repeat => {}
        }
    }

    fn mouse_move_event(&mut self, x: f64, y: f64) {
        self.input.mouse_position = Some((x, y));
    }

    fn mouse_scroll_event(&mut self, x: f64, y: f64) {
        self.input.mouse_scrolled = Some((x, y));
    }

    fn close_event(&mut self, window: &mut PWindow) {
        window.set_should_close(true);
    }

    fn framebuffer_size_event(&mut self, width: i32, height: i32) {
        self.frame_buffer_size = (width, height);
    }

    fn focus_event(&mut self, focused: bool) {
        self.focus = focused;
    }

    fn iconify_event(&mut self, iconified: bool) {
        self.iconified = iconified;
    }

    fn maximize_event(&mut self, maximized: bool) {
        self.maximized = maximized;
    }

    fn file_drop_event(&mut self, paths: Vec<PathBuf>) {
        self.file_dropped = Some(paths);
    }

    fn cursor_enter_event(&mut self, entered: bool) {
        self.cursor_entered = entered;
    }

    pub fn clear_input_events(&mut self) {
        self.input.key_pressed.clear();
        self.input.key_released.clear();

        self.input.mouse_scrolled = None;

        self.input.mouse_button_pressed.clear();
        self.input.mouse_button_released.clear();
    }
}
