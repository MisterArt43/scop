use std::mem::offset_of;

use gl::{
    ActiveTexture, BindTexture, COLOR_BUFFER_BIT, Clear, ClearColor, DEPTH_BUFFER_BIT,
    DrawElements, TEXTURE_2D, TEXTURE0, TEXTURE1,
};
use glfw::{Context, ffi::glfwSwapBuffers};

use crate::gfx::*;
use crate::image::{BMP, PPM};
use crate::math::vec2::Vec2;
use crate::texture::Texture;
use crate::{
    app::app::Application,
    gfx::{shader::Shader, vbo::VBO},
    math::vec3::Vec3,
};

pub mod app;
pub mod gfx;
pub mod image;
pub mod math;

#[repr(C)]
struct VertexTuto {
    position: Vec3,
    color: Vec3,
    uv: Vec2,
}

fn main() {
    let mut app = Application::new("Scop", 800.0, 800.0);

    /*================================================================================================
     *                                         INIT MAIN LOOP
     *================================================================================================**/

    let vertices = [
        VertexTuto {
            position: Vec3::new(0.5, 0.5, 0.0),
            color: Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            uv: Vec2 { x: 1.0, y: 1.0 },
        },
        VertexTuto {
            position: Vec3::new(0.5, -0.5, 0.0),
            color: Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            uv: Vec2 { x: 1.0, y: 0.0 },
        },
        VertexTuto {
            position: Vec3 {
                x: -0.5,
                y: -0.5,
                z: 0.0,
            },
            color: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            uv: Vec2 { x: 0.0, y: 0.0 },
        },
        VertexTuto {
            position: Vec3 {
                x: -0.5,
                y: 0.5,
                z: 0.0,
            },
            color: Vec3 {
                x: 1.0,
                y: 1.0,
                z: 0.0,
            },
            uv: Vec2 { x: 0.0, y: 1.0 },
        },
    ];

    let indices: [u32; 6] = [0, 1, 3, 1, 2, 3];

    /*======================
     *    VBO
     *========================**/

    let vbo = VBO::new();
    vbo.upload(&vertices, BufferUsage::Static);

    /*======================
     *    VERTEX SHADER
     *========================**/

    let vertex_shader_source = "shader/tuto.vert";

    /*======================
     *    FRAGMENT SHADER
     *========================**/

    let fragment_shader_source = "shader/tuto.frag";

    /*======================
     *    SHADER PROGRAM
     *========================**/

    let shader_program = Shader::new(&vertex_shader_source, &fragment_shader_source)
        .expect("Failed to create shader program");
    shader_program.activate();

    /*======================
     *    VERTEX ARRAY OBJECT
     *========================**/

    let layout = VertexLayout::new::<VertexTuto>()
        .push::<Vec3>(offset_of!(VertexTuto, position))
        .push::<Vec3>(offset_of!(VertexTuto, color))
        .push::<Vec2>(offset_of!(VertexTuto, uv));

    let vao = VAO::new();
    vao.set_layout(&vbo, &layout);

    /*======================
     *    EBO
     *========================**/

    let ebo = EBO::new();
    ebo.upload(&indices, BufferUsage::Static);

    vao.bind();
    ebo.bind();
    VAO::unbind();

    /*======================
     *    Texture
     *========================**/

    let mut img1 = PPM::load("C:\\Users\\arthu\\Documents\\GitHub\\scop\\ressources\\fd.ppm")
        .expect("Error couldn't load image");
    img1.flipv();

    let mut texture = Texture::new_2d();
    texture
        .set_wrap(texture::TextureWrap::Repeat, texture::TextureWrap::Repeat)
        .set_filter(
            texture::TextureFilter::LinearMipmapLinear,
            texture::TextureFilter::Linear,
        );

    texture
        .upload(
            img1.width(),
            img1.height(),
            img1.pixels(),
            texture::TextureFormat::RGB8,
        )
        .generate_mipmaps();

    let mut img2 = BMP::load("C:\\Users\\arthu\\Pictures\\mikutransparent.bmp")
        .expect("Error couldn't load image");
    img2.flipv();

    let mut texture2 = Texture::new_2d();
    texture2
        .set_wrap(texture::TextureWrap::Repeat, texture::TextureWrap::Repeat)
        .set_filter(
            texture::TextureFilter::LinearMipmapLinear,
            texture::TextureFilter::Linear,
        );

    texture2
        .upload(
            img2.width(),
            img2.height(),
            img2.pixels(),
            texture::TextureFormat::RGBA8,
        )
        .generate_mipmaps();

    // * Defining the texture units for the shader * //
    shader_program.activate();
    shader_program.set_uniform_int("texture1", 0);
    shader_program.set_uniform_int("texture2", 1);

    /*========================
     *    MAIN LOOP
     *========================**/

    while !app.window.should_close() {
        // ! *================== Handle Events =================*/
        app.process_events();
        app.update_delta_time();

        // ! *================== Clear Screen =================*/
        unsafe {
            ClearColor(0.2, 0.3, 0.3, 1.0);
            Clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
        }

        // ! *================== Uniform ==============*/
        // unsafe {
        //     let time_value = glfwGetTime() as f32;
        //     let green_value = [0.0, (time_value.sin() / 2.0) + 0.5, 0.0, 1.0];
        //     shader_program.activate();
        //     shader_program.set_uniform_vec4("ourColor", &green_value);
        // }

        // ! *================== Draw =================*/
        shader_program.activate();
        unsafe {
            ActiveTexture(TEXTURE0);
            BindTexture(TEXTURE_2D, texture.id());
            ActiveTexture(TEXTURE1);
            BindTexture(TEXTURE_2D, texture2.id());
        }
        vao.bind();

        unsafe {
            DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }

        // ! *================== Swap Buffers =================*/
        unsafe {
            glfwSwapBuffers(app.window.window_ptr());
        }
    }
}
