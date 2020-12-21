extern crate gl;
use self::gl::types::*;

use std::ffi::c_void;
use std::mem;

#[derive(Debug, Clone)]
pub struct Vbo {
    vbo_id: GLuint,
}

#[allow(dead_code)]
impl Vbo {
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

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo_id);
        }
    }

    pub fn add_data<T>(&self, data_element_number: usize, data_pointer: *const c_void, draw_mode: GLenum) {
        self.bind();

        unsafe {
            gl::BufferData(gl::ARRAY_BUFFER, (data_element_number * mem::size_of::<T>()) as GLsizeiptr, data_pointer, draw_mode);
        }
    }
}