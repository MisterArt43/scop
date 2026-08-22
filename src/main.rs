use gl::{COLOR_BUFFER_BIT, Clear, ClearColor};
use glfw::{Context, ffi::glfwSwapBuffers};

use crate::{app::app::Application, gfx::{shader::Shader, vbo::VBO}};
use crate::gfx::*;

pub mod app;
pub mod gfx;

fn main() {
    let mut app = Application::new("Scop", 800.0, 800.0);


    /*================================================================================================
     *                                         INIT MAIN LOOP
     *================================================================================================**/

    let vertices : [f32; 9] = [
        -0.5, -0.5, 0.0,
         0.5, -0.5, 0.0,
         0.0,  0.5, 0.0
    ]; 

    /*======================
     *    VBO
     *========================**/

    let vbo = VBO::new();
    vbo.upload(&vertices, BufferUsage::Static);

    /*======================
     *    VERTEX SHADER
     *========================**/

    let vertex_shader_source = concat!(
        "#version 330 core\n",
        "layout (location = 0) in vec3 aPos;\n",
        "void main()\n",
        "{\n",
        "   gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);\n",
        "}\0"
    );

    /*======================
     *    FRAGMENT SHADER
     *========================**/

    let fragment_shader_source = concat!(
        "#version 330 core\n",
        "out vec4 FragColor;\n",
        "void main()\n",
        "{\n",
        "   FragColor = vec4(1.0, 0.5, 0.2, 1.0);\n",
        "}\0"
    );
    
    /*======================
    *    SHADER PROGRAM
    *========================**/
    
    let shader_program = Shader::new(vertex_shader_source, fragment_shader_source)
        .expect("Failed to create shader program");
    shader_program.activate();

    

    /*========================
     *    MAIN LOOP
     *========================**/

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
