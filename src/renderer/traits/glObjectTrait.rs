extern crate gl;
use self::gl::types::*;

use crate::renderer::traits::vaoTrait::VaoLayoutTrait;
use crate::renderer::traits::vboTrait::VboTrait;
use crate::renderer::traits::eboTrait::OptionalEboTrait;
use crate::renderer::constants;
use crate::renderer::properties::GlObjectProperties;

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

    /*  This function a link to the add_data function in renderer::vbo.
    *   We cannot access the vbo in the GlObject so we must expose the function.
    *   We also need to bind the vao.
    */
    fn add_vertex_data<T>(&self, data_element_number: usize, data_pointer: *const c_void, draw_mode: GLenum) {
        self.bind_vao();
        self.add_vbo_data::<T>(data_element_number, data_pointer, draw_mode);
    }
    
    /*  This function is similar to the add_vertex_data function.
    *   It is a a link to the add_data function in renderer::ebo.
    *   We cannot access the ebo in the GlObject so we must expose the function.
    *   It also creates a new ebo if it isn't set.
    *   We also need to bind the vao.
    */
    fn add_index_data<T>(&mut self, data_element_number: usize, data_pointer: *const c_void, draw_mode: GLenum) {        
        if self.get_ebo_id().is_some() {
            self.bind_vao();
            self.add_ebo_data::<T>(data_element_number, data_pointer, draw_mode);
        }    
    }

    /*  This function is used to set a property in the properties struct inside the GlObject class.
    *   It takes the property_id that is a constant defined in renderer::constants and the value to set.
    *   It matches the property_id and sets the right property.
    *   ***I should really find a more dynamic way to set the property maybe using an array... idk...***
    */
    fn set_property(&mut self, property_id: u8, property: GLenum) {
        let properties = self.get_mut_properties_ref();

        match property_id {
            constants::DRAW_MODE_PROPERTY => properties.draw_mode = property,
            constants::EBO_TYPE_PROPERTY  => properties.ebo_type  = property,
            _ => println!("Wrong property set on GlObject."),
        }
    }

    /*  This is a function used to easily draw thigs on the screen.
    *   It uses the function raw_draw_elements if an ebo is set.
    *   Or it uses the function raw_draw_arrays if the ebo is not set.
    *   It takes the number of vertices to draw.
    *   Note: it does not draw things by itself, but it calls raw_draw_elements() and raw_draw_arrays().
    */
    fn draw(&self, count: GLint) { //smart way to draw.
        match self.get_ebo_id() {
            Some(_) => { //We have a Element Buffer Object. We can use gl::DrawElements().
                self.raw_draw_elements(self.get_properties_ref().draw_mode, count, self.get_properties_ref().ebo_type);
            },
            None => { //We have a Vertex Buffer Object. We must use gl::DrawArrays().
                self.raw_draw_arrays(self.get_properties_ref().draw_mode, count);
            },
        }
    }

    /*  This function is used to draw elements on the screen.
    *   It binds the vao and the vbo and then calls gl::DrawElements().
    */
    fn raw_draw_elements(&self, mode: GLenum, count: GLsizei, type_: GLenum) {
        self.bind_vao();
        self.bind_vbo();
        unsafe {
            gl::DrawElements(mode, count, type_, ptr::null());
        }
    }

    /*  This function is used to draw arrays on the screen.
    *   It binds the vao and the vbo and then calls gl::DrawArrays().
    */
    fn raw_draw_arrays(&self, mode: GLenum, count: GLsizei) {
        self.bind_vao();
        self.bind_vbo();
        unsafe {
            gl::DrawArrays(mode, 0, count);
        }
    }
}