/*  File: renderer/globject.rs
*   Author: Vicix
*
*   This file contains a class called GlObject.
*   GlObject holds a vao, a vbo and a ebo. 
*   It makes easier to organize and draw things on the screen.
*/

extern crate gl;
use self::gl::types::*;

use crate::renderer::traits::vboTrait::VboTrait;
use crate::renderer::traits::vaoTrait::{VaoTrait, VaoLayoutTrait};
use crate::renderer::traits::eboTrait::OptionalEboTrait;
use crate::renderer::traits::glObjectTrait::GlObjectTrait;

use crate::renderer::vaoLayoutElement::VaoLayoutElement;
use crate::renderer::glObjectProperties::GlObjectProperties;


/*  The structure definition.
*   It contains a vao, vbo and eventually an ebo.
*   Additionally it contains the properties of the Object.
*/
pub struct GlObject {
    pub vao_id: GLuint,
    layout: Vec::<VaoLayoutElement>,
    vbo_id: GLuint,
    ebo_id: Option<GLuint>,
    properties: GlObjectProperties,
}

impl GlObject {
    pub fn new() -> GlObject {
        GlObject {
            vao_id: {
                let mut vao_id = 1;
                unsafe {
                    gl::GenVertexArrays(1, &mut vao_id);
                    gl::BindVertexArray(vao_id);
                }

                vao_id
            },
            layout: Vec::<VaoLayoutElement>::new(),
            vbo_id: {
                let mut vbo_id = 1;
                unsafe {
                    gl::GenBuffers(1, &mut vbo_id);
                    gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id);
                }

                vbo_id
            },
            ebo_id: None,
            properties: GlObjectProperties {
                draw_mode: gl::TRIANGLES,
                ebo_type: gl::UNSIGNED_INT,
            },
        }
    }

    pub fn with_ebo() -> GlObject {
        GlObject {
            vao_id: {
                let mut vao_id = 1;
                unsafe {
                    gl::GenVertexArrays(1, &mut vao_id);
                    gl::BindVertexArray(vao_id);
                }

                vao_id
            },
            layout: Vec::<VaoLayoutElement>::new(),
            vbo_id: {
                let mut vbo_id = 1;
                unsafe {
                    gl::GenBuffers(1, &mut vbo_id);
                    gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id);
                }

                vbo_id
            },
            ebo_id: {
                let mut ebo = 1;
                unsafe {
                    gl::GenBuffers(1, &mut ebo);
                    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
                }

                Some(ebo)
            },
            properties: GlObjectProperties {
                draw_mode: gl::TRIANGLES,
                ebo_type: gl::UNSIGNED_INT,
            },
        }
    }
}

impl VboTrait for GlObject {
    fn get_vbo_id(&self) -> u32 {
        self.vbo_id
    }
}

impl VaoTrait for GlObject {
    fn get_vao_id(&self) -> u32 {
        self.vao_id
    }
}

impl VaoLayoutTrait for GlObject {
    fn get_layout_ref(&self) -> &Vec::<VaoLayoutElement> {
        &self.layout
    }

    fn get_mut_layout_ref(&mut self) -> &mut Vec::<VaoLayoutElement> {
        &mut self.layout
    }
}

impl OptionalEboTrait for GlObject {
    fn get_ebo_id(&self) -> Option<u32> {
        self.ebo_id
    }
}

impl GlObjectTrait for GlObject {
    fn get_properties_ref(&self) -> &GlObjectProperties {
        &self.properties
    }

    fn get_mut_properties_ref(&mut self) -> &mut GlObjectProperties {
        &mut self.properties
    }
}

/*#[allow(dead_code)]
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
            properties: GlObjectProperties {
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
            properties: GlObjectProperties {
                draw_mode: gl::TRIANGLES,
                ebo_type: gl::UNSIGNED_INT,
            },
        }
    }

    /*  Another constructor.
    *   This time we construct our GlObject from an existing and vbo.
    *   Like the function new() and the function from_vao_and_vbo() we don't want to assing an ebo and we set the property to the default. 
    */
    pub fn add_existing_ebo(&mut self, ebo: &ebo::Ebo) {
        self.ebo = Some(ebo.clone());
    }

    /*  This function a link to the add_data function in renderer::vbo.
    *   We cannot access the vbo in the GlObject so we must expose the function.
    *   We also need to bind the vao.
    */
    pub fn add_vertex_data<T>(&self, data_element_number: usize, data_pointer: *const c_void, draw_mode: GLenum) {
        self.vao.bind_vao();
        self.vbo.bind_vbo();
        self.vbo.add_vbo_data::<T>(data_element_number, data_pointer, draw_mode);
    }

    /*  This function is similar to the add_vertex_data function.
    *   It is a a link to the add_data function in renderer::ebo.
    *   We cannot access the ebo in the GlObject so we must expose the function.
    *   It also creates a new ebo if it isn't set.
    *   We also need to bind the vao.
    */
    pub fn add_index_data<T>(&mut self, data_element_number: usize, data_pointer: *const c_void, draw_mode: GLenum) {
        self.vao.bind_vao();
        
        if self.ebo.is_none() { //If the ebo is not set we have to set a new one!
            self.ebo = Some(ebo::Ebo::new());
        }

        self.ebo.as_ref().unwrap().bind_ebo();
        self.ebo.as_ref().unwrap().add_ebo_data::<T>(data_element_number, data_pointer, draw_mode);
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
        self.vbo.bind_vbo();
        self.vao.write_layout();
    }

    /*  This function is used to set a property in the properties struct inside the GlObject class.
    *   It takes the property_id that is a constant defined in renderer::constants and the value to set.
    *   It matches the property_id and sets the right property.
    *   ***I should really find a more dynamic way to set the property maybe using an array... idk...***
    */
    pub fn set_property(&mut self, property_id: u8, property: GLenum) {
        match property_id {
            constants::DRAW_MODE_PROPERTY => self.properties.draw_mode = property,
            constants::EBO_TYPE_PROPERTY  => self.properties.ebo_type  = property,
            _ => println!("Wrong property set on GlObject."),
        }
    }

    /*  This is a function used to easily draw thigs on the screen.
    *   It uses the function raw_draw_elements if an ebo is set.
    *   Or it uses the function raw_draw_arrays if the ebo is not set.
    *   It takes the number of vertices to draw.
    *   Note: it does not draw things by itself, but it calls raw_draw_elements() and raw_draw_arrays().
    */
    pub fn draw(&self, count: GLint) { //smart way to draw.
        self.vao.bind_vao();
        self.vbo.bind_vbo();

        match self.ebo {
            Some(_) => { //We have a Element Buffer Object. We can use gl::DrawElements().
                self.raw_draw_elements(self.properties.draw_mode, count, self.properties.ebo_type);
            },
            None => { //We have a Vertex Buffer Object. We must use gl::DrawArrays().
                self.raw_draw_arrays(self.properties.draw_mode, count);
            },
        }
    }

    /*  This function is used to draw elements on the screen.
    *   It binds the vao and the vbo and then calls gl::DrawElements().
    */
    pub fn raw_draw_elements(&self, mode: GLenum, count: GLsizei, type_: GLenum) {
        self.vao.bind_vao();
        self.vbo.bind_vbo();
        unsafe {
            gl::DrawElements(mode, count, type_, ptr::null());
        }
    }

    /*  This function is used to draw arrays on the screen.
    *   It binds the vao and the vbo and then calls gl::DrawArrays().
    */
    pub fn raw_draw_arrays(&self, mode: GLenum, count: GLsizei) {
        self.vao.bind_vao();
        self.vbo.bind_vbo();
        unsafe {
            gl::DrawArrays(mode, 0, count);
        }
    }
}*/