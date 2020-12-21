extern crate glfw;
use self::glfw::{Context, Key, Action};

extern crate gl;
use self::gl::types::*;

use std::sync::mpsc::Receiver;
use std::os::raw::c_void;
use std::ffi::CStr;

mod renderer;

use crate::renderer::shader;
use crate::renderer::texture;
use crate::renderer::constants;
use crate::renderer::globject;
use crate::renderer::material;

// settings
const SCR_WIDTH: u32 = 600;
const SCR_HEIGHT: u32 = 600;


pub fn main() {
    // glfw: initialize and configure
    // ------------------------------
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    // glfw window creation
    // --------------------
    let (mut window, events) = glfw.create_window(SCR_WIDTH, SCR_HEIGHT, "LearnOpenGL", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // gl: load all OpenGL function pointers
    // ---------------------------------------
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    unsafe {
        println!("Opengl Version: {}", CStr::from_ptr(gl::GetString(gl::VERSION) as *const i8).to_str().unwrap());

        let mut nr_attributes = 1;
        gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut nr_attributes);
        println!("Maximum nr of vertex attributes supported: {}", nr_attributes);
    }


    let vertices: [f32; 20] = [
        -0.5,  0.7,  0.0,  1.0,
         0.5,  0.7,  1.0,  1.0,
         0.0,  0.0,  0.5,  0.5,
        -0.5, -0.7,  0.0,  0.0,
         0.5, -0.7,  1.0,  0.0,
    ];

    let indices = [
        0, 1, 2,
        2, 4, 3,
    ];

    let shader = shader::Shader::new("./src/shaders/vert2.glsl", "./src/shaders/frag2.glsl");

    let mut globj = globject::GlObject::new();
    globj.add_vertex_data::<GLfloat>(vertices.len(), &vertices[0] as *const f32 as *const c_void, gl::STATIC_DRAW);
    globj.add_index_data::<GLint>(indices.len(), &indices[0] as *const i32 as *const c_void, gl::STATIC_DRAW);
    globj.push_layout_element(gl::FLOAT, gl::FALSE, 2);
    globj.push_layout_element(gl::FLOAT, gl::FALSE, 2);
    globj.set_property(constants::DRAW_MODE_PROPERTY, gl::TRIANGLES);
    globj.set_property(constants::EBO_TYPE_PROPERTY, gl::UNSIGNED_INT);
    globj.write_layout();

    let mut texture1 = texture::Texture::new(gl::TEXTURE_2D, "./src/resources/wall.jpg", gl::RGB, gl::RGB, 0);
    texture1.set_gl_property(gl::TEXTURE_MAG_FILTER, gl::LINEAR);
    texture1.set_gl_property(gl::TEXTURE_MIN_FILTER, gl::LINEAR);
    texture1.gen_texture();

    let mut texture2 = texture::Texture::new(gl::TEXTURE_2D, "./src/resources/awesomeface.png", gl::RGB, gl::RGBA, 1);
    texture2.set_gl_property(gl::TEXTURE_MAG_FILTER, gl::LINEAR);
    texture2.set_gl_property(gl::TEXTURE_MIN_FILTER, gl::LINEAR);
    texture2.set_image_property(constants::FLIP_V_PROPERTY, 1);
    texture2.gen_texture();

    let mut texture3 = texture::Texture::new(gl::TEXTURE_2D, "./src/resources/cat.jpeg", gl::RGB, gl::RGB, 2);
    texture3.set_image_property(constants::FLIP_V_PROPERTY, 1);
    texture3.gen_texture();

    let mut material = material::Material::new(&shader);
    material.push_texture(&texture1, "texture1");
    material.push_texture(&texture2, "texture2");
    material.push_texture(&texture3, "texture3");


    let mut position = 0.5;
    let mut mode = true;

    // render loop
    // -----------
    while !window.should_close() {
        // events
        // -----
        process_events(&mut window, &events);

        unsafe {
            gl::ClearColor(0.2, 0.5, 0.8, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            if mode {
                position += 0.0001;
            } else {
                position -= 0.0001;
            }
            if position >= 1.0 || position <= -1.0 {
                mode = !mode;
            }

            material.bind();
            material.set_float_uniform("position", position);
            globj.draw(6);
            material.set_float_uniform("position", -position);
            globj.draw(6);
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        // -------------------------------------------------------------------------------
        window.swap_buffers();
        glfw.poll_events();
    }
}

// NOTE: not the same version as in common.rs!
fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            _ => {}
        }
    }
}