use gl::{CULL_FACE, Viewport};

use crate::{application::Application, shader::Shader};

pub mod application;
pub mod bmp;
pub mod camera;
pub mod ebo;
pub mod material;
pub mod math;
pub mod mesh;
pub mod shader;
pub mod texture;
pub mod vao;
pub mod vbo;

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!(
            "Usage: {} <vertex_shader> <fragment_shader> <obj_file>",
            args[0]
        );
        println!("Using default shaders and mesh...");
        args.push(String::from("./shader/solid.vert"));
        args.push(String::from("./shader/solid.frag"));
    }
    if args.len() < 4 {
        args.push(String::from("./ressources/42.obj"));
    }
    /*
     * Step 1 creation de la fenetre (glfw) et
     * du contexte pour opengl (gl_loader / gl)
     */
    let mut app = Application::new("Scop", 800.0, 800.0);

    /*
     * Step 2 creation du shader (read --> compile --> link a CG)
     */
    let vert_file = String::from(args[1].clone());
    let frag_file = String::from(args[2].clone());

    println!(
        "\n=== Chargement du shader ===\n Vertex: {}\nFragment: {}",
        vert_file, frag_file
    );
    let shader = Shader::new(&vert_file, &frag_file).expect("Failed to load Shader files");
    // applique le shader (active le shader pour que les uniform(variables) et les textures soient pris en compte)
    shader.activate();

    /*
     * Step 3 creation du mesh (parsing du .obj et chargement des differentes data dans les buffers
     * (VBO ,VAO ,EBO))
     * VBO : Vertex Buffer Object, buffer qui stocke les vertices du mesh
     * VAO : Vertex Array Object, buffer qui stocke les configurations des attributs de vertex (en gros on definit l'ordre et les types des variables passées en buffer)
     * EBO : Element Buffer Object, buffer qui stocke les indices des vertices pour le dessin du mesh (permet de reutiliser les vertices voisin et d'eviter les duplications))
     */

    let mut obj_data = mesh::Mesh::from_obj(&args[3]).expect("Failed to load mesh");
    // obj_data.push(
    //     mesh::Mesh::from_obj("./ressources/42.obj").expect("Failed to load material data")[0]
    //         .clone(),
    // );
    println!(
        "\nMesh loaded with {} vertices and {} faces",
        obj_data
            .iter()
            .map(|submesh| submesh.mesh.nb_vertices)
            .sum::<usize>(),
        obj_data
            .iter()
            .map(|submesh| submesh.mesh.index_count)
            .sum::<usize>()
    );

    // ========================================================================
    // Charger les textures en GPU
    // ========================================================================
    println!("\n=== Chargement des textures en GPU ===");
    for submesh in &mut obj_data {
        if let Some(mtl) = &mut submesh.material_data {
            // Charger la texture diffuse
            if let Some(texture) = &mut mtl.diffuse_texture_data {
                match texture.load_to_gpu() {
                    Ok(id) => println!(
                        "✓ Texture diffuse du matériau '{}' chargée (ID: {})",
                        mtl.name, id
                    ),
                    Err(e) => eprintln!("✗ Erreur lors du chargement de la texture diffuse: {}", e),
                }
            }

            // Charger la texture spéculaire
            if let Some(texture) = &mut mtl.specular_texture_data {
                match texture.load_to_gpu() {
                    Ok(id) => println!(
                        "✓ Texture spéculaire du matériau '{}' chargée (ID: {})",
                        mtl.name, id
                    ),
                    Err(e) => eprintln!(
                        "✗ Erreur lors du chargement de la texture spéculaire: {}",
                        e
                    ),
                }
            }
        }
    }
    println!("=== Textures chargées ===\n");

    app.camera.init_view(&obj_data);

    shader.set_uniform_mat4("model", &app.camera.model.to_flat_array());

    unsafe {
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
        gl::Enable(CULL_FACE);
    }

    // compute camera distance after scaling so big objects don't push the camera too far

    while !app.window.should_close() {
        app.update_delta_time();
        let (fb_width, fb_height) = app.window.get_framebuffer_size();

        // Set up matrices BEFORE rendering
        if fb_height > 0 {
            app.camera
                .update_view_and_projection(fb_width as f32, fb_height as f32);

            shader.set_uniform_mat4("projection", &app.camera.projection.to_flat_array());
            shader.set_uniform_mat4("view", &app.camera.view.to_flat_array());

            let eye_pos = app.camera.eye_position(); // + Vec3::new(0.0, 1.0, 0.0);
            shader.set_uniform_vec3("lightPos", &eye_pos.to_array());
            shader.set_uniform_vec3("viewPos", &eye_pos.to_array());
            shader.set_uniform_vec3("lightColor", &[1.0, 1.0, 1.0]);
        }

        // /*
        //temp shadertoy setter
        unsafe {
            // Pour iTime
            let i_time_loc = gl::GetUniformLocation(shader.id, "iTime\0".as_ptr() as *const i8);
            gl::Uniform1f(i_time_loc, app.last_frame_time);

            // Pour iResolution
            let i_resolution_loc =
                gl::GetUniformLocation(shader.id, "iResolution\0".as_ptr() as *const i8);
            gl::Uniform3f(i_resolution_loc, fb_width as f32, fb_height as f32, 1.0);

            // Pour iMouse
            let i_mouse_loc = gl::GetUniformLocation(shader.id, "iMouse\0".as_ptr() as *const i8);
            gl::Uniform4f(i_mouse_loc, app.curpos.0, app.curpos.1, 0.0, 0.0);
        }
        // end temp
        // */
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        for submesh in &obj_data {
            if let Some(mtl) = &submesh.material_data {
                shader.set_uniform_vec3("materialDiffuse", &mtl.diffuse_color);
                shader.set_uniform_vec3("materialSpecular", &mtl.specular_color);
                shader.set_uniform_float("materialShininess", mtl.shininess);

                // ====================================================================
                // Binder et utiliser la texture diffuse si elle existe
                // ====================================================================
                if let Some(texture) = &mtl.diffuse_texture_data {
                    if texture.texture_id.is_some() {
                        texture.bind(0); // Bind au slot 0
                        shader.set_uniform_int("diffuseTexture", 0); // Dire au shader d'utiliser le slot 0
                        shader.set_uniform_bool("hasTexture", true);
                    } else {
                        shader.set_uniform_bool("hasTexture", false);
                    }
                } else {
                    shader.set_uniform_bool("hasTexture", false);
                }
            } else {
                shader.set_uniform_vec3("materialDiffuse", &[1.0, 1.0, 1.0]);
                shader.set_uniform_vec3("materialSpecular", &[1.0, 1.0, 1.0]);
                shader.set_uniform_float("materialShininess", 32.0);
                shader.set_uniform_bool("hasTexture", false);
            }
            submesh.mesh.draw();
        }

        app.swap_buffers();
        unsafe {
            Viewport(0, 0, fb_width, fb_height);
        }

        app.handle_events();
    }

    for submesh in &obj_data {
        submesh.mesh.delete();

        // Supprimer les textures de la GPU
        if let Some(mtl) = &submesh.material_data {
            if let Some(texture) = &mtl.diffuse_texture_data {
                texture.delete();
            }
            if let Some(texture) = &mtl.specular_texture_data {
                texture.delete();
            }
        }
    }
}
