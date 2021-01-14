/*  File: renderer/vao.rs
*   Author: Vicix
*
*   This file contains the Vao class .
*   The class Vao is a simple abstaction of the opengl vao. 
*   It helps creating vao and setting the vertex attrib pointers.
*/

extern crate gl;
use self::gl::types::*;

use crate::renderer::traits::vaoTrait::{VaoTrait, VaoLayoutTrait};
use crate::renderer::vaoLayoutElement::VaoLayoutElement;

/*  This is the declaration on the class.
*   It contains the vao_id and a vector of VaoLayoutElements. Used for setting the vertex attrb pointers.
*/
#[derive(Clone)]
pub struct Vao {
    vao_id: GLuint,
    layout: Vec::<VaoLayoutElement>,
}

#[allow(dead_code)]
impl Vao{
    /*  The constructor of the class.
    *   Creates a vap and creates a new VaoLayoutElement vector.
    */
    pub fn new() -> Vao {
        let vao = unsafe {
            let mut vao = 1;
            gl::GenVertexArrays(1, &mut vao);
            
            vao
        };
        
        Vao {
            vao_id: vao,
            layout: Vec::<VaoLayoutElement>::new(),
        }
    }
}

impl VaoTrait for Vao {
    fn get_vao_id(&self) -> u32 {
        self.vao_id
    }
}

impl VaoLayoutTrait for Vao {
    fn get_layout_ref(&self) -> &Vec::<VaoLayoutElement> {
        &self.layout
    }

    fn get_mut_layout_ref(&mut self) -> &mut Vec::<VaoLayoutElement> {
        &mut self.layout
    }
}