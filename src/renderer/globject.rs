/*  File: renderer/globject.rs
*   Author: Vicix
*
*   This file contains a class called GlObject.
*   GlObject holds a vao, a vbo and a ebo. 
*   It makes easier to organize and draw things on the screen.
*/

extern crate gl;
use self::gl::types::*;

use crate::renderer::vao;
use crate::renderer::vbo;
use crate::renderer::ebo;
use crate::renderer::constants;

use std::ffi::c_void;
use std::ptr;

/*  The structure definition.
*   It contains a vao, vbo and eventually an ebo.
*   Additionally it contains the properties of the Object.
*/
pub struct GlObject {
    vao: vao::Vao,
    vbo: vbo::Vbo,
    ebo: Option<ebo::Ebo>,
    properties: Properties,
}

#[allow(dead_code)]
impl GlObject {
    /*  The main constructor of the class. 
    *   We create a new vao and a new vbo.
    *   We don't want to assign an ebo, because it is not necessary.
    *   We'll assign it only when we need it.
    *   Also properties are set as gl::TRIANGLES and gl::UNSIGNED_INT by default.
    *   Those will likely be overritten by the user using the function set_property()
    */
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

    /*  Another constructor.
    *   This time we construct our GlObject from an existing vao and vbo.
    *   Like the function new() we don't want to assing an ebo and we set the property to the default. 
    */
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

    /*  Another constructor.
    *   This time we construct our GlObject from an existing and vbo.
    *   Like the function new() and the function from_vao_and_vbo() we don't want to assing an ebo and we set the property to the default. 
    */
    pub fn from_vbo(vbo: &vbo::Vbo) -> GlObject {
        GlObject {
            vao: vao::Vao::new(),
            vbo: vbo.clone(),
            ebo: None,
            properties: Properties {
                draw_mode: gl::TRIANGLES,
                ebo_type: gl::UNSIGNED_INT,
            },
        }
    }

    /*  This function is used to add an existing ebo to the GlObject.
    *   It fail if there is already an ebo added.
    *   It is useful if we created the object using the function from_vao_and_vbo()
    */
    pub fn link_ebo(&mut self, ebo: &ebo::Ebo) {
        if self.ebo.is_none() {
            self.ebo = Some(ebo.clone());
        } else {
            println!("The GlObject has already assigned an ebo!!! Aborting.");
        }
    }

    /*  This function a link to the add_data function in renderer::vbo.
    *   We cannot access the vbo in the GlObject so we must expose the function.
    *   We also need to bind the vao.
    */
    pub fn add_vertex_data<T>(&self, data_element_number: usize, data_pointer: *const c_void, draw_mode: GLenum) {
        self.vao.bind();
        self.vbo.add_data::<T>(data_element_number, data_pointer, draw_mode);
    }

    /*  This function is similar to the add_vertex_data function.
    *   It is a a link to the add_data function in renderer::ebo.
    *   We cannot access the ebo in the GlObject so we must expose the function.
    *   It also creates a new ebo if it isn't set.
    *   We also need to bind the vao.
    */
    pub fn add_index_data<T>(&mut self, data_element_number: usize, data_pointer: *const c_void, draw_mode: GLenum) {
        self.vao.bind();
        
        if self.ebo.is_none() {
            self.ebo = Some(ebo::Ebo::new(&self.vao));
        }

        self.ebo.as_ref().unwrap().add_data::<T>(data_element_number, data_pointer, draw_mode);
    }

    /*  This function is a link to the push_layout_element function in renderer::vao.
    *   We cannot access the vao in the GlObject so we must expose the function.
    */
    pub fn push_layout_element(&mut self, element_type: GLenum, normalized: GLboolean, element_count: GLint) {
        self.vao.push_layout_element(element_type, normalized, element_count);
    }

    /*  This function is a link to the set_layout_element function in renderer::vao.
    *   We cannot access the vao in the GlObject so we must expose the function.
    */
    pub fn set_layout_element(&mut self, element_type: GLenum, normalized: GLboolean, element_count: GLint, attrb_array_number: usize) {
        self.vao.set_layout_element(element_type, normalized, element_count, attrb_array_number);
    }

    /*  This function is a link to the pop_layout_element function in renderer::vao.
    *   We cannot access the vao in the GlObject so we must expose the function.
    */
    pub fn pop_layout_element(&mut self) {
        self.vao.pop_layout_element();
    }

    /*  This function is a link to the clear_layout function in renderer::vao.
    *   We cannot access the vao in the GlObject so we must expose the function.
    */
    pub fn clear_layout(&mut self) {
        self.vao.clear_layout();
    }

    /*  This function is a link to the write_layout function in renderer::vao.
    *   We cannot access the vao in the GlObject so we must expose the function.
    *   We also need to bind the vao.
    */
    pub fn write_layout(&self) {
        self.vbo.bind();
        self.vao.write_layout();
    }

    /*  This function is used to set a property in the properties struct inside the GlObject class.
    *   It takes the property_id that is a constant defined in renderer::constants and the value to set.
    *   It matches the property_id and sets the right property.
    *   ***I should really find a more dynamic way to set the property maybe using an array... idk...***
    */
    pub fn set_property(&mut self, property_id: u8, value: GLenum) {
        match property_id {
            constants::DRAW_MODE_PROPERTY => self.properties.draw_mode = value,
            constants::EBO_TYPE_PROPERTY  => self.properties.ebo_type  = value,
            _ => println!("Wrong property set on GlObject."),
        }
    }

    /*  This is a function used to easily draw thigs on the screen.
    *   It uses the function raw_draw_elements if an ebo is set.
    *   Or it uses the function raw_draw_arrays if the ebo is not set.
    *   It takes the number of vertices to draw.
    *   Note: it does not draw things by itself, but it calls raw_draw_elements() and raw_draw_arrays().
    */
    pub fn draw(&self, count: GLint) {
        self.vao.bind();
        self.vbo.bind();

        if self.ebo.is_some() {
            self.raw_draw_elements(self.properties.draw_mode, count, self.properties.ebo_type);
        } else {
            self.raw_draw_arrays(self.properties.draw_mode, count);
        }
    }

    /*  This function is used to draw elements on the screen.
    *   It binds the vao and the vbo and then calls gl::DrawElements().
    */
    pub fn raw_draw_elements(&self, mode: GLenum, count: GLsizei, type_: GLenum) {
        self.vao.bind();
        self.vbo.bind();
        unsafe {
            gl::DrawElements(mode, count, type_, ptr::null());
        }
    }

    /*  This function is used to draw arrays on the screen.
    *   It binds the vao and the vbo and then calls gl::DrawArrays().
    */
    pub fn raw_draw_arrays(&self, mode: GLenum, count: GLsizei) {
        self.vao.bind();
        self.vbo.bind();
        unsafe {
            gl::DrawArrays(mode, 0, count);
        }
    }

    pub fn as_vbo_ref(&self) -> &vbo::Vbo {
        &self.vbo
    }

    pub fn as_vao_ref(&self) -> &vao::Vao {
        &self.vao
    }

    pub fn as_ebo_ref(&self) -> &Option<ebo::Ebo> {
        &self.ebo
    }
}

/*  This struct contains the properties that are used to draw.
*   ***I should find a more modular system***
*/
#[derive(Clone)]
struct Properties {
    draw_mode: GLenum,
    ebo_type: GLenum,
}