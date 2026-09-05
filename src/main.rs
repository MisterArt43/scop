use std::path::Path;

use gl::{
    Clear,
    ClearColor,
    Enable,
    COLOR_BUFFER_BIT,
    DEPTH_BUFFER_BIT,
    DEPTH_TEST,
};

use glfw::{Context, ffi::glfwSwapBuffers};

use crate::{
    app::app::Application, asset::MaterialLoader, camera::camera::Camera, gfx::{
        material::Material,
        shader::Shader,
    }, math::transform, mesh::Actor,
};

pub mod app;
pub mod asset;
pub mod gfx;
pub mod image;
pub mod math;
pub mod mesh;
pub mod camera;

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

    let obj_path = project_root.join(
        "ressources/Center City Sci-Fi.obj"
    );

    let vertex_shader_path = project_root.join(
        "shader/tuto.vert"
    );

    let fragment_shader_path = project_root.join(
        "shader/tuto.frag"
    );


    /*==============================================================*/
    /*                         Actor                                */
    /*==============================================================*/

    let actor = Actor::from_obj(
        obj_path
            .to_str()
            .expect("Invalid OBJ path"),
    )
    .expect("Failed to load OBJ");


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

    let object_dir = obj_path
        .parent()
        .unwrap_or(project_root);

    let mut material_loader =
        MaterialLoader::new(object_dir);

    let materials: Vec<Material> = actor
        .submeshes
        .iter()
        .map(|submesh| {
            match &submesh.material_data {
                Some(mtl) => material_loader
                    .load(mtl)
                    .expect("Failed to load material"),

                None => Material::new("default"),
            }
        })
        .collect();

    /*============================================
     *               CAMERA
     *=============================================**/

    let transform = transform::Transform::new();
    let projection = crate::camera::projection::Projection::perspective(
        45.0,
        800.0 / 800.0,
        0.1,
        100.0,
    );
    let mut camera = Camera::new(transform, projection);
    let mut controller = crate::camera::controller::CameraController::new(
        5.0,
        0.01,
    );

    /*==============================================================*/
    /*                         Main loop                            */
    /*==============================================================*/

    while !app.window.should_close() {
        /*==================== Events ====================*/

        app.process_events();
        app.update_delta_time();

        controller.update(&mut camera, &app.event_handler.input, app.deltatime());


        /*===================== Clear ====================*/

        unsafe {
            ClearColor(
                0.2,
                0.3,
                0.3,
                1.0,
            );

            Clear(
                COLOR_BUFFER_BIT
                    | DEPTH_BUFFER_BIT
            );
        }


        /*===================== Shader ===================*/

        shader.activate();


        /*====================== Draw ====================*/

        for (submesh, material) in actor
            .submeshes
            .iter()
            .zip(materials.iter())
        {
            material.apply(&shader);

            submesh.draw();
        }


        /*================== Swap buffers ================*/

        unsafe {
            glfwSwapBuffers(
                app.window.window_ptr()
            );
        }
    }
}