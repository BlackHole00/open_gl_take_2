extern crate gl;
use self::gl::types::*;

use crate::renderer::vao;
use crate::renderer::vbo;
use crate::renderer::ebo;
use crate::renderer::constants;

use std::ffi::c_void;
use std::ptr;

pub struct GlObject {
    vao: vao::Vao,
    vbo: vbo::Vbo,
    ebo: Option<ebo::Ebo>,
    properties: Properties,
}

#[allow(dead_code)]
impl GlObject {
    pub fn new() -> GlObject {
        GlObject {
            vao: vao::Vao::new(),
            vbo: vbo::Vbo::new(),
            ebo: None,
            properties: Properties {
                draw_mode: gl::TRIANGLES,
                ebo_type: gl::UNSIGNED_INT,
            },
        }
    }

    pub fn from_vao_and_vbo(vao: &vao::Vao, vbo: &vbo::Vbo) -> GlObject {
        GlObject {
            vao: vao.clone(),
            vbo: vbo.clone(),
            ebo: None,
            properties: Properties {
                draw_mode: gl::TRIANGLES,
                ebo_type: gl::UNSIGNED_INT,
            },
        }
    }

    pub fn add_existing_ebo(&mut self, ebo: &ebo::Ebo) {
        self.ebo = Some(ebo.clone());
    }

    pub fn add_vertex_data<T>(&self, data_element_number: usize, data_pointer: *const c_void, draw_mode: GLenum) {
        self.vao.bind();
        self.vbo.bind();
        self.vbo.add_data::<T>(data_element_number, data_pointer, draw_mode);
    }

    pub fn add_index_data<T>(&mut self, data_element_number: usize, data_pointer: *const c_void, draw_mode: GLenum) {
        self.vao.bind();
        
        if self.ebo.is_none() { //If the ebo is not set we have to set a new one!
            self.ebo = Some(ebo::Ebo::new());
        }

        self.ebo.as_ref().unwrap().bind();
        self.ebo.as_ref().unwrap().add_data::<T>(data_element_number, data_pointer, draw_mode);
    }

    pub fn push_layout_element(&mut self, element_type: GLenum, normalized: GLboolean, element_count: GLint) {
        self.vao.push_layout_element(element_type, normalized, element_count);
    }

    pub fn set_layout_element(&mut self, element_type: GLenum, normalized: GLboolean, element_count: GLint, attrb_array_number: usize) {
        self.vao.set_layout_element(element_type, normalized, element_count, attrb_array_number);
    }

    pub fn pop_layout_element(&mut self) {
        self.vao.pop_layout_element();
    }

    pub fn clear_layout(&mut self) {
        self.vao.clear_layout();
    }

    pub fn write_layout(&self) {
        self.vbo.bind();
        self.vao.write_layout();
    }

    pub fn set_property(&mut self, property_id: u8, property: GLenum) {
        match property_id {
            constants::DRAW_MODE_PROPERTY => self.properties.draw_mode = property,
            constants::EBO_TYPE_PROPERTY  => self.properties.ebo_type  = property,
            _ => println!("Wrong property set on GlObject."),
        }
    }

    pub fn draw(&self, count: GLint) { //smart way to draw.
        self.vao.bind();
        self.vbo.bind();

        match self.ebo {
            Some(_) => { //We have a Element Buffer Object. We can use gl::DrawElements().
                self.raw_draw_elements(self.properties.draw_mode, count, self.properties.ebo_type);
            },
            None => { //We have a Vertex Buffer Object. We must use gl::DrawArrays().
                self.raw_draw_arrays(self.properties.draw_mode, count);
            },
        }
    }

    pub fn raw_draw_elements(&self, mode: GLenum, count: GLsizei, type_: GLenum) {
        self.vao.bind();
        self.vbo.bind();
        unsafe {
            gl::DrawElements(mode, count, type_, ptr::null());
        }
    }

    pub fn raw_draw_arrays(&self, mode: GLenum, count: GLsizei) {
        self.vao.bind();
        self.vbo.bind();
        unsafe {
            gl::DrawArrays(mode, 0, count);
        }
    }
}

#[derive(Clone)]
struct Properties {
    draw_mode: GLenum,
    ebo_type: GLenum,
}