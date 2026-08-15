use std::{collections::HashSet, path::PathBuf};

use glfw::PWindow;

pub struct AppEvent {
    pub input: InputEvent,
    pub frame_buffer_size: (i32, i32),

}

pub struct InputEvent {
    pub key_pressed: HashSet<glfw::Key>,
    pub key_released: HashSet<glfw::Key>,
    pub mouse_moved: Option<(f64, f64)>,
    pub mouse_scrolled: Option<(f64, f64)>,
    pub mouse_button_pressed: HashSet<glfw::MouseButton>,
    pub mouse_button_released: HashSet<glfw::MouseButton>,
    pub modifiers: glfw::Modifiers,
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
}

impl AppEvent {
    pub fn new() -> Self {
        AppEvent {
            input: InputEvent::new(),
        }
    }

    pub fn handle_event(&mut self, event: glfw::WindowEvent, window: PWindow) {
        match event {
            glfw::WindowEvent::CursorEnter(enter) => {
                self.cursor_enter_event(enter);
            }
            glfw::WindowEvent::Key(key, _scancode, action, mods) => {
                self.key_event(key, action, mods);
            }
            glfw::WindowEvent::MouseButton(button, action, mods) => {
                self.mouse_button_event(button, action, mods);
            }
            glfw::WindowEvent::CursorPos(x, y) => {
                self.mouse_move_event(x, y);
            }
            glfw::WindowEvent::Scroll(x, y) => {
                self.mouse_scroll_event(x, y);
            }
            glfw::WindowEvent::Close => {
                self.close_event(window);
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
            glfw::WindowEvent::FileDrop(paths) => {
                self.file_drop_event(paths);
            }
            _ => {}
        }
    }

    fn key_event(&mut self, key: glfw::Key, action: glfw::Action, mods: glfw::Modifiers) {
        self.input.modifiers = mods; // Met à jour le masque de modificatrices

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

    fn mouse_button_event(
        &mut self,
        button: glfw::MouseButton,
        action: glfw::Action,
        mods: glfw::Modifiers,
    ) {
        self.input.modifiers = mods; // Met également à jour sur les clics de souris

        match action {
            glfw::Action::Press => {
                self.input.mouse_button_pressed.insert(button);
                self.input.mouse_button_released.remove(&button);
            }
            glfw::Action::Release => {
                self.input.mouse_button_released.insert(button);
                self.input.mouse_button_pressed.remove(&button);
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

    fn close_event(&mut self, mut window: PWindow) {
        window.set_should_close(true);
    }

    fn framebuffer_size_event(&mut self, _width: i32, _height: i32) {

    }

    fn focus_event(&mut self, _focused: bool) {}
    fn iconify_event(&mut self, _iconified: bool) {}
    fn file_drop_event(&mut self, _paths: Vec<PathBuf>) {}
    fn cursor_enter_event(&mut self, _entered: bool) {}


    pub fn clear_input_events(&mut self) {
        self.input.key_pressed.clear();
        self.input.key_released.clear();
        self.input.mouse_moved = None;
        self.input.mouse_scrolled = None;
        self.input.mouse_button_pressed.clear();
        self.input.mouse_button_released.clear();
        // Remarque : on conserve self.input.modifiers car l'utilisateur peut maintenir Shift/Ctrl enfoncé sur plusieurs frames.
    }
}