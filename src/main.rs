use gl::{ClearDepth, DEPTH_TEST, DepthFunc, Disable, Enable};
use gl_loader::init_gl;
use glfw::{self, Context, WindowHint};

use crate::{application::Application, mesh::Vertex, shader::Shader};

pub mod application;
pub mod ebo;
pub mod material;
pub mod mesh;
pub mod shader;
pub mod texture;
pub mod vao;
pub mod vbo;

fn main() {
    let vert: Vec<Vertex> = Vec::from([
        Vertex {
            position: [-0.9, -0.9, -0.9],
            normal: [1.0, 0.0, 0.0],
            uv: [0.0, 0.0],
        },
        Vertex {
            position: [0.0, 0.9, 0.0],
            normal: [0.0, 0.0, 1.0],
            uv: [0.5, 1.0],
        },
        Vertex {
            position: [0.9, -0.9, -0.9],
            normal: [0.0, 1.0, 0.0],
            uv: [1.0, 0.0],
        },
    ]);

    /*[[-0.5, 0.0, -0.5],
    [0.0, 0.5, 0.0],
    [0.5, 0.0, -0.5]].to_vec();*/

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
    let indices: Vec<u32> = vec![0, 1, 2];
    let mesh = mesh::Mesh::new(&vert, &indices);

    while !app.window.should_close() {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        mesh.draw();
        app.swap_buffers();

        app.handle_events();
    }
}
