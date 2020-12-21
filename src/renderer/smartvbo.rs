extern crate gl;
use self::gl::types::*;

use crate::renderer::vbo;
use std::ffi::c_void;

pub struct SmartVbo<T> {
    vbo: vbo::Vbo,
    vertices: Vec::<T>,
}

#[allow(dead_code)]
impl<T> SmartVbo<T> {
    pub fn new() -> SmartVbo<T> {
        SmartVbo {
            vbo: vbo::Vbo::new(),
            vertices: Vec::<T>::new(),
        }
    }

    pub fn from_vbo(vbo: &vbo::Vbo) -> SmartVbo<T> {
        SmartVbo {
            vbo: vbo.clone(),
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
        self.vbo.add_data::<T>(self.vertices.len(), &self.vertices[0] as *const T as *const c_void, draw_mode);
    }

    pub fn as_vbo_ref(&self) -> &vbo::Vbo {
        &self.vbo
    }

    pub fn bind(&self) {
        self.vbo.bind();
    }
}