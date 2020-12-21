extern crate gl;
use self::gl::types::*;

use crate::renderer::ebo;
use crate::renderer::vao;
use std::ffi::c_void;

pub struct SmartEbo<T> {
    ebo: ebo::Ebo,
    linked_vao: vao::Vao, 
    vertices: Vec::<T>,
}

#[allow(dead_code)]
impl<T> SmartEbo<T> {
    pub fn new(vao: &vao::Vao) -> SmartEbo<T> {
        SmartEbo {
            linked_vao: vao.clone(),
            ebo: ebo::Ebo::new(vao),
            vertices: Vec::<T>::new(),
        }
    }

    pub fn from_ebo(vao: &vao::Vao, ebo: &ebo::Ebo) -> SmartEbo<T> {
        SmartEbo {
            linked_vao: vao.clone(),
            ebo: ebo.clone(),
            vertices: Vec::<T>::new(),
        }
    }


    pub fn push_data(&mut self, value: T) {
        self.vertices.push(value);
    }

    pub fn pop_data(&mut self) {
        self.vertices.pop();
    }

    pub fn clear_data(&mut self) {
        self.vertices = Vec::<T>::new();
    }

    pub fn write_data(&self, draw_mode: GLenum) {
        self.linked_vao.bind();
        self.ebo.add_data::<T>(self.vertices.len(), &self.vertices[0] as *const T as *const c_void, draw_mode);
    }

    pub fn as_ebo_ref(&self) -> &ebo::Ebo {
        &self.ebo
    }

    pub fn bind(&self) {
        self.ebo.bind();
    }
}