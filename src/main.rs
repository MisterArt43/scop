use gl::{Clear, ClearColor, COLOR_BUFFER_BIT};
use glfw::{ffi::glfwSwapBuffers, Context};

use crate::app::app::Application;

pub mod app;

fn main() {
    let mut app = Application::new("Scop", 800.0, 800.0);

    while !app.window.should_close() {
        app.handle_events();
        app.update_delta_time();

        unsafe {
            ClearColor(0.2, 0.3, 0.3, 1.0);
            Clear(COLOR_BUFFER_BIT);
        }

        unsafe {
            glfwSwapBuffers(app.window.window_ptr());
        }
    }
}
