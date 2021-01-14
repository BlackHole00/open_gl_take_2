extern crate gl;
use self::gl::types::*;
use std::ffi::c_void;
use std::mem;

pub trait EboTrait {
    fn get_ebo_id(&self) -> u32;
    
    /*  A simple function to bind the buffer.
    */
    fn bind_ebo(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.get_ebo_id());
        }
    }

    /*  A simple function to add the data to the buffer.
    */
    fn add_ebo_data<T>(&self, data_element_number: usize, data_pointer: *const c_void, draw_mode: GLenum) {
        self.bind_ebo();

        unsafe {
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (data_element_number * mem::size_of::<T>()) as GLsizeiptr, data_pointer, draw_mode);
        }
    }
}

pub trait OptionalEboTrait {
    fn get_ebo_id(&self) -> Option<u32>;

    /*  A simple function to bind the buffer.
    *   If the ebo is None() it does nothing.
    */
    fn bind_ebo(&self) {
        match self.get_ebo_id() {
            Some(ebo_id) => unsafe {
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo_id);
            },
            None => {},
        }
    }

    /*  A simple function to add the data to the buffer.
    *   If the ebo is None() it does nothing.
    */
    fn add_ebo_data<T>(&self, data_element_number: usize, data_pointer: *const c_void, draw_mode: GLenum) {
        match self.get_ebo_id() {
            Some(_) => unsafe {
                self.bind_ebo();
                gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (data_element_number * mem::size_of::<T>()) as GLsizeiptr, data_pointer, draw_mode);
            },
            None => {},
        }
    }
}