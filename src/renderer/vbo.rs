/*  File: renderer/vbo.rs
*   Author: Vicix
*
*   This file contains the Vbo class .
*   The class Vbo is a simple abstaction of the opengl vbo. 
*/
extern crate gl;
use self::gl::types::*;

use crate::renderer::traits::vboTrait::VboTrait;


/*  The declaration of the class.
*   The only thing we need is the vbo id.
*/
#[derive(Debug, Clone)]
pub struct Vbo {
    vbo_id: GLuint,
}

#[allow(dead_code)]
impl Vbo {
    /*  The constructor of the class.
    *   It generates a vbo and saves the id.
    */
    pub fn new() -> Vbo {
        let mut vbo = 1;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        }
        
        Vbo {
            vbo_id: vbo,
        }
    }


    /*  A simple function to add the data to the buffer.
    *
    pub fn add_data<T>(&self, data_element_number: usize, data_pointer: *const c_void, draw_mode: GLenum) {
        self.bind_vbo();

        unsafe {
            gl::BufferData(gl::ARRAY_BUFFER, (data_element_number * mem::size_of::<T>()) as GLsizeiptr, data_pointer, draw_mode);
        }
    }*/
}

impl VboTrait for Vbo {
    fn get_vbo_id(&self) -> u32 {
        self.vbo_id
    }
}