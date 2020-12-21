extern crate gl;
use self::gl::types::*;

use std::ffi::c_void;
use std::mem;

#[derive(Debug, Clone)]
pub struct Ebo {
    ebo_id: GLuint,
}

#[allow(dead_code)]
impl Ebo {
    pub fn new() -> Ebo {
        let mut ebo = 1;
        unsafe {
            gl::GenBuffers(1, &mut ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        }
        
        Ebo {
            ebo_id: ebo,
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo_id);
        }
    }

    pub fn add_data<T>(&self, data_element_number: usize, data_pointer: *const c_void, draw_mode: GLenum) {
        self.bind();

        unsafe {
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (data_element_number * mem::size_of::<T>()) as GLsizeiptr, data_pointer, draw_mode);
        }
    }
}