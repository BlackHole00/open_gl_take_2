extern crate gl;
use self::gl::types::*;

use std::ffi::c_void;
use std::mem;

pub trait VboTrait {
    fn get_vbo_id(&self) -> u32;

    /*  A simple function to bind the buffer.
    */
    fn bind_vbo(&self) {
        let vbo_id = self.get_vbo_id();

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id);
        }
    }

    /*  A simple function to add the data to the buffer.
    */
    fn add_vbo_data<T>(&self, data_element_number: usize, data_pointer: *const c_void, draw_mode: GLenum) {
        self.bind_vbo();

        unsafe {
            gl::BufferData(gl::ARRAY_BUFFER, (data_element_number * mem::size_of::<T>()) as GLsizeiptr, data_pointer, draw_mode);
        }
    }
}