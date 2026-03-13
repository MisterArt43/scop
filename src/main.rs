use std::ffi::CString;

use gl::Viewport;

use crate::{application::Application, shader::Shader};

pub mod application;
pub mod ebo;
pub mod material;
pub mod mesh;
pub mod shader;
pub mod texture;
pub mod vao;
pub mod vbo;
pub mod math;
pub mod camera;


fn main() {
    
    /*
     * Step 1 creation de la fenetre (glfw) et
     * du contexte pour opengl (gl_loader / gl)
     */
    let mut app = Application::new("Scop", 800.0, 800.0);

    /*
     * Step 2 creation du shader (read --> compile --> link a CG)
     */
    let shader = Shader::new("./shader/basic.vert", "./shader/basic.frag")
    .expect("Failed to load Shader files");
    // applique le shader (active le shader pour que les uniform(variables) et les textures soient pris en compte)
    shader.activate();

    /*
     * Step 3 creation du mesh (parsing du .obj et chargement des differentes data dans les buffers
     * (VBO ,VAO ,EBO))
     * VBO : Vertex Buffer Object, buffer qui stocke les vertices du mesh
     * VAO : Vertex Array Object, buffer qui stocke les configurations des attributs de vertex (en gros on definit l'ordre et les types des variables passées en buffer)
     * EBO : Element Buffer Object, buffer qui stocke les indices des vertices pour le dessin du mesh (permet de reutiliser les vertices voisin et d'eviter les duplications))
     */

    let mut args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        args.push(String::from("./ressources/42.obj"));
    }

    let obj_data = mesh::Mesh::from_obj(&args[1]).expect("Failed to load mesh");

    while !app.window.should_close() {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        for submesh in &obj_data {
            submesh.mesh.draw();
        }
        app.swap_buffers();
        
        let location = unsafe { gl::GetUniformLocation(shader.id, CString::new("aspect_ratio").unwrap().as_ptr()) };
        app.camera.update_basic_rot();
        let (fb_width, fb_height) = app.window.get_framebuffer_size();
        unsafe {
            Viewport(0, 0, fb_width, fb_height);
        }

        if fb_height > 0 {
            app.camera.set_aspect_ratio(fb_width as f32 / fb_height as f32);
        }

        app.handle_events();
    }

    for submesh in &obj_data {
        submesh.mesh.delete();
    }
}

