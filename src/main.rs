/*  File: main.rs
*   Author: Vicix
*
*   This is the main file of this project.
*   This is mainly used as testground. It will be poorly commented.
*/
extern crate glfw;
use self::glfw::{Context, Key, Action};

extern crate gl;
use self::gl::types::*;

use std::sync::mpsc::Receiver;
use std::ffi::CStr;

mod renderer;

use crate::renderer::shader;
use crate::renderer::texture;
use crate::renderer::constants;
use crate::renderer::globject;
use crate::renderer::material;
use crate::renderer::smartvbo;
use crate::renderer::smartebo;

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
    
    
    let shader = shader::Shader::new("./src/shaders/vert.glsl", "./src/shaders/frag.glsl");
    
    //I should make this a macro...
    //I create a smart vbo and set the vertecies data.
    let mut smart_vbo = smartvbo::SmartVbo::<GLfloat>::new();
    smart_vbo.push_data(-0.5); smart_vbo.push_data( 0.7); smart_vbo.push_data(0.0); smart_vbo.push_data(1.0);
    smart_vbo.push_data( 0.5); smart_vbo.push_data( 0.7); smart_vbo.push_data(1.0); smart_vbo.push_data(1.0);
    smart_vbo.push_data( 0.0); smart_vbo.push_data( 0.0); smart_vbo.push_data(0.5); smart_vbo.push_data(0.5);
    smart_vbo.push_data(-0.5); smart_vbo.push_data(-0.7); smart_vbo.push_data(0.0); smart_vbo.push_data(0.0);
    smart_vbo.push_data( 0.5); smart_vbo.push_data(-0.7); smart_vbo.push_data(1.0); smart_vbo.push_data(0.0);
    //Then we send the data to the cpu
    smart_vbo.write_data(gl::STATIC_DRAW);
    //And we create a globject using it.
    let mut globj = globject::GlObject::from_vbo(smart_vbo.as_vbo_ref());
    
    //Here we create a smart ebo 
    let mut smart_ebo = smartebo::SmartEbo::<GLuint>::new(globj.as_vao_ref());
    //and add the vertices.
    smart_ebo.push_data(0); smart_ebo.push_data(1); smart_ebo.push_data(2);
    smart_ebo.push_data(2); smart_ebo.push_data(4); smart_ebo.push_data(3); 
    smart_ebo.write_data(gl::STATIC_DRAW);
    //and we link the ebo with the globj
    globj.link_ebo(smart_ebo.as_ebo_ref());
    
    //Then we need to push a layout element
    globj.push_layout_element(gl::FLOAT, gl::FALSE, 2);
    globj.push_layout_element(gl::FLOAT, gl::FALSE, 2);
    //And set the draw property
    globj.set_property(constants::DRAW_MODE_PROPERTY, gl::TRIANGLES);
    globj.set_property(constants::EBO_TYPE_PROPERTY, gl::UNSIGNED_INT);
    //And write the layout
    globj.write_layout();

    //Here we create the textures from file and set the property
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

    //Then we link the shader and the textures to a material.
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
            material.set_float_uniform("position_x", position);
            material.set_float_uniform("position_y", position);
            globj.draw(6);
            /*material.set_float_uniform("position_x", position);
            material.set_float_uniform("position_y", -position);
            globj.draw(6);
            material.set_float_uniform("position_x", -position);
            material.set_float_uniform("position_y", position);
            globj.draw(6);
            material.set_float_uniform("position_x", -position);
            material.set_float_uniform("position_y", -position);
            globj.draw(6);
            
            material.set_float_uniform("position_x", position / 2.0);
            material.set_float_uniform("position_y", position / 2.0);
            globj.draw(6);
            material.set_float_uniform("position_x", position / 2.0);
            material.set_float_uniform("position_y", -position / 2.0);
            globj.draw(6);
            material.set_float_uniform("position_x", -position / 2.0);
            material.set_float_uniform("position_y", position / 2.0);
            globj.draw(6);
            material.set_float_uniform("position_x", -position / 2.0);
            material.set_float_uniform("position_y", -position / 2.0);
            globj.draw(6);

            material.set_float_uniform("position_x", position);
            material.set_float_uniform("position_y", 0.0);
            globj.draw(6);
            material.set_float_uniform("position_x", -position);
            globj.draw(6);
            material.set_float_uniform("position_x", 0.0);
            material.set_float_uniform("position_y", position);
            globj.draw(6);
            material.set_float_uniform("position_y", -position);
            globj.draw(6);*/
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        // -------------------------------------------------------------------------------
        window.swap_buffers();
        glfw.poll_events();
    }
}

//Event Processing
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