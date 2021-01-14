extern crate gl;
use self::gl::types::*;

use crate::renderer::traits::vaoTrait::VaoLayoutTrait;
use crate::renderer::traits::vboTrait::VboTrait;
use crate::renderer::traits::eboTrait::OptionalEboTrait;
use crate::renderer::constants;
use crate::renderer::glObjectProperties::GlObjectProperties;

use std::ffi::c_void;
use std::ptr;

pub trait GlObjectTrait: VaoLayoutTrait + VboTrait + OptionalEboTrait {
    fn get_properties_ref(&self) -> &GlObjectProperties;
    fn get_mut_properties_ref(&mut self) -> &mut GlObjectProperties;
    
    fn bind_gl_object(&self) {
        self.bind_vao();
        self.bind_ebo();
        self.bind_vbo();
    }

    fn add_vertex_data<T>(&self, data_element_number: usize, data_pointer: *const c_void, draw_mode: GLenum) {
        self.bind_vao();
        //self.bind_vbo();
        self.add_vbo_data::<T>(data_element_number, data_pointer, draw_mode);
    }
    
    fn add_index_data<T>(&mut self, data_element_number: usize, data_pointer: *const c_void, draw_mode: GLenum) {        
        if self.get_ebo_id().is_some() {
            self.bind_vao();
            
            //self.bind_ebo();
            self.add_ebo_data::<T>(data_element_number, data_pointer, draw_mode);
        }    
    }

    fn set_property(&mut self, property_id: u8, property: GLenum) {
        let properties = self.get_mut_properties_ref();

        match property_id {
            constants::DRAW_MODE_PROPERTY => properties.draw_mode = property,
            constants::EBO_TYPE_PROPERTY  => properties.ebo_type  = property,
            _ => println!("Wrong property set on GlObject."),
        }
    }

    fn draw(&self, count: GLint) { //smart way to draw.
        //self.bind_vao();
        //self.bind_vbo();

        match self.get_ebo_id() {
            Some(_) => { //We have a Element Buffer Object. We can use gl::DrawElements().
                self.raw_draw_elements(self.get_properties_ref().draw_mode, count, self.get_properties_ref().ebo_type);
            },
            None => { //We have a Vertex Buffer Object. We must use gl::DrawArrays().
                self.raw_draw_arrays(self.get_properties_ref().draw_mode, count);
            },
        }
    }

    fn raw_draw_elements(&self, mode: GLenum, count: GLsizei, type_: GLenum) {
        self.bind_vao();
        self.bind_vbo();
        unsafe {
            gl::DrawElements(mode, count, type_, ptr::null());
        }
    }

    fn raw_draw_arrays(&self, mode: GLenum, count: GLsizei) {
        self.bind_vao();
        self.bind_vbo();
        unsafe {
            gl::DrawArrays(mode, 0, count);
        }
    }
}