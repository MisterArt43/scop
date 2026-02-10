
use gl::{ClearDepth, DEPTH_TEST, DepthFunc, Disable, DrawArrays, Enable, FLOAT, TRIANGLES, types::GLsizei};
use gl_loader::init_gl;
use glfw::{self, Context, WindowHint};

use crate::{shader::Shader, vao::VAO, vbo::VBO};

pub mod vbo;
pub mod vao;
pub mod shader;


fn main() {
    let vert: Vec<[f32; 3]> = [[-0.5, 0.0, -0.5],
            [0.0, 0.5, 0.0],
            [0.5, 0.0, -0.5]].to_vec();

    let mut glfw = glfw::init(error_callback).unwrap();


    WindowHint::ContextVersion(3, 3);
    WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core);
    WindowHint::DepthBits(Some(24));
    WindowHint::Samples(Some(4));

    let width = 800;
    let height = 800;
    let title = "scop";
    let (mut window, events) = glfw.create_window(width, height, title, glfw::WindowMode::Windowed).expect("Failed to create windows");

    
    window.set_key_polling(true);
    window.make_current();
    
    init_gl();
    gl::load_with(|s| window.get_proc_address(s).map_or(std::ptr::null(), |f| f as *const _));
    
    unsafe {
        Enable(DEPTH_TEST);
        DepthFunc(gl::LESS);
        ClearDepth(1.0);
        Disable(gl::CULL_FACE);
        Enable(gl::MULTISAMPLE);
    }
    
    let shader = Shader::new("./shader/basic.vert", "./shader/basic.frag").expect("Failed to load Shader files");

    let vbo = VBO::new(&vert, size_of_val(&vert).try_into().expect("failed to cast into usize (Overflow)"));
    let vao= VAO::new();

    vao.bind();
    vao.link_attrib(&vbo, 0, 3, FLOAT, size_of::<[i32; 3]>() as GLsizei, 0);

    vao.unbind();
    vbo.unbind();



    while !window.should_close() {

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        
        shader.activate();
        vao.bind();
        unsafe {
            DrawArrays(TRIANGLES, 0, 3);
            // DrawElements(TRIANGLES, 0, UNSIGNED_INT, null());
        }
        window.swap_buffers();

        if let Some((_id, _event)) = events.receive() {
            
        }
    }

}   

fn error_callback(err: glfw::Error, description: String) {
    eprintln!("GLFW error {:?}: {:?}", err, description);
}