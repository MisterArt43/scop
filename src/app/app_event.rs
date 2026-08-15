use std::collections::HashSet;

pub struct AppEvent {
    input: InputEvent,
}

pub struct InputEvent {
    pub key_pressed: HashSet<glfw::Key>,
    pub key_released: HashSet<glfw::Key>,
    pub mouse_moved: Option<(f64, f64)>,
    pub mouse_scrolled: Option<(f64, f64)>,
    pub mouse_button_pressed: HashSet<glfw::MouseButton>,
    pub mouse_button_released: HashSet<glfw::MouseButton>,
}

impl AppEvent {
    pub fn new() -> Self {
        AppEvent {
            input: InputEvent::new(),
        }
    }

    pub fn handle_event(&mut self, event: glfw::WindowEvent) {
        match event {
            glfw::WindowEvent::Key(key, scancode, action, mods) => {
                self.key_event(key, action);
            }
            glfw::WindowEvent::CursorPos(x, y) => {
                self.mouse_move_event(x, y);
            }
            glfw::WindowEvent::Scroll(x, y) => {
                self.mouse_scroll_event(x, y);
            }
            glfw::WindowEvent::Close => {
                self.close_event();
            }
            glfw::WindowEvent::FramebufferSize(width, height) => {
                self.resize_event(width, height);
            }
            glfw::WindowEvent::Focus(focused) => {
                self.focus_event(focused);
            }
            glfw::WindowEvent::Iconify(iconified) => {
                self.iconify_event(iconified);
            }
        }
    }

    fn key_event(&mut self, key: glfw::Key, action: glfw::Action) {
        match action {
            glfw::Action::Press => {
                self.input.key_pressed.insert(key);
                self.input.key_released.remove(&key);
            }
            glfw::Action::Release => {
                self.input.key_released.insert(key);
                self.input.key_pressed.remove(&key);
            }
            _ => {}
        }
    }

    fn mouse_move_event(&mut self, x: f64, y: f64) {
        self.input.mouse_moved = Some((x, y));
    }

    fn mouse_scroll_event(&mut self, x: f64, y: f64) {
        self.input.mouse_scrolled = Some((x, y));
    }

    fn close_event(&mut self) {
        // Handle window close event
    }

    fn resize_event(&mut self, width: i32, height: i32) {
        // Handle window resize event
    }

    fn focus_event(&mut self, focused: bool) {
        // Handle window focus event
    }

    fn iconify_event(&mut self, iconified: bool) {
        // Handle window iconify event
    }

    pub fn clear_input_events(&mut self) {
        self.input.key_pressed.clear();
        self.input.key_released.clear();
        self.input.mouse_moved = None;
        self.input.mouse_scrolled = None;
        self.input.mouse_button_pressed.clear();
        self.input.mouse_button_released.clear();
    }

}

impl InputEvent {
    fn new() -> Self {
        InputEvent {
            key_pressed: HashSet::new(),
            key_released: HashSet::new(),
            mouse_moved: None,
            mouse_scrolled: None,
            mouse_button_pressed: HashSet::new(),
            mouse_button_released: HashSet::new(),
        }
    }
}