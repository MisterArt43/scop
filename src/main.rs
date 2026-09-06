use std::path::Path;

use gl::{Clear, ClearColor, Enable, Viewport, COLOR_BUFFER_BIT, DEPTH_BUFFER_BIT, DEPTH_TEST};

use glfw::{
    ffi::{glfwSwapBuffers, glfwWindowHint},
    Context,
};

use crate::{
    app::app::Application,
    asset::MaterialLoader,
    camera::{camera::Camera, projection::Projection},
    gfx::{material::Material, shader::Shader},
    math::transform,
    mesh::Actor,
};

pub mod app;
pub mod asset;
pub mod camera;
pub mod gfx;
pub mod image;
pub mod math;
pub mod mesh;

fn main() {
    /*
     * Application / OpenGL context
     */
    let mut app = Application::new("Scop", 800.0, 800.0);

    unsafe {
        Enable(DEPTH_TEST);
    }

    /*==============================================================*/
    /*                         Paths                                */
    /*==============================================================*/

    let project_root = Path::new(env!("CARGO_MANIFEST_DIR"));

    let mut obj_path = project_root.join("ressources/Center City Sci-Fi.obj");

    let vertex_shader_path = project_root.join("shader/unlit.vert");

    let fragment_shader_path = project_root.join("shader/unlit.frag");

    let args = std::env::args().collect::<Vec<String>>();
    let first_arg = args.get(1);

    if !first_arg.is_none() {
        let arg = first_arg.unwrap();
        let path = Path::new(arg);

        if path.exists() && path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "obj" {
                    println!("Loading OBJ file: {}", arg);
                    obj_path = path.to_path_buf();
                } else {
                    println!("Invalid file extension: {}", ext.to_string_lossy());
                }
            } else {
                println!("No file extension found for: {}", arg);
            }
        } else {
            println!("File does not exist or is not a file: {}", arg);
        }
    }

    /*==============================================================*/
    /*                         Actor                                */
    /*==============================================================*/

    let actor =
        Actor::from_obj(obj_path.to_str().expect("Invalid OBJ path")).expect("Failed to load OBJ");

    /*==============================================================*/
    /*                         Shader                               */
    /*==============================================================*/

    let shader = Shader::new(
        vertex_shader_path
            .to_str()
            .expect("Invalid vertex shader path"),
        fragment_shader_path
            .to_str()
            .expect("Invalid fragment shader path"),
    )
    .expect("Failed to create shader");

    /*==============================================================*/
    /*                       Materials                              */
    /*==============================================================*/

    let object_dir = obj_path.parent().unwrap_or(project_root);

    let mut material_loader = MaterialLoader::new(object_dir);

    let materials: Vec<Material> = actor
        .submeshes
        .iter()
        .map(|submesh| match &submesh.material_data {
            Some(mtl) => material_loader.load(mtl).expect("Failed to load material"),

            None => Material::new("default"),
        })
        .collect();

    /*============================================
     *               CAMERA
     *=============================================**/

    let transform = transform::Transform::new();
    let projection =
        crate::camera::projection::Projection::perspective(45.0, 800.0 / 800.0, 0.1, 100.0);
    let mut camera = Camera::new(transform, projection);
    let mut controller = crate::camera::controller::CameraController::new(5.0, 0.01);

    /*==============================================================*/
    /*                         Main loop                            */
    /*==============================================================*/

    while !app.window.should_close() {
        /*==================== Events ====================*/

        app.update_delta_time();
        app.process_events();

        let (framebuffer_width, framebuffer_height) = app.event_handler.frame_buffer_size;
        if framebuffer_width > 0 && framebuffer_height > 0 {
            unsafe {
                Viewport(0, 0, framebuffer_width, framebuffer_height);
            }
        }

        /*================== CAMERA =================*/

        camera.update_projection(Projection::Perspective {
            fov_y: 80.0,
            aspect_ratio: framebuffer_width as f32 / framebuffer_height as f32,
            near: 0.001,
            far: 1000.0,
        });
        controller.update(&mut camera, &app.event_handler.input, app.deltatime());

        /*===================== Clear ====================*/

        unsafe {
            ClearColor(0.0, 0.0, 0.0, 1.0);

            Clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
        }

        // ! *================== Uniform ==============*/
        shader.activate();
        shader.set_uniform_mat4("view", &camera.view_mat().to_flat_array());
        shader.set_uniform_mat4("projection", &camera.projection_mat().to_flat_array());
        shader.set_uniform_mat4("model", &actor.model_matrix().to_flat_array());

        /*===================== Shader ===================*/

        shader.activate();

        /*====================== Draw ====================*/

        for (submesh, material) in actor.submeshes.iter().zip(materials.iter()) {
            material.apply(&shader);

            submesh.draw();
        }

        /*================== Swap buffers ================*/

        unsafe {
            glfwSwapBuffers(app.window.window_ptr());
        }
    }
}
