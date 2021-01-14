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
use crate::renderer::properties::GlObjectProperties;


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

    /*  The main constructor of the class. 
    *   We create a new vao, a new vbo and a new ebo.
    *   Also properties are set as gl::TRIANGLES and gl::UNSIGNED_INT by default.
    *   Those will likely be overritten by the user using the function set_property()
    */
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