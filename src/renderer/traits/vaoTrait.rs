extern crate gl;
use self::gl::types::*;

use crate::renderer::vaoLayoutElement::VaoLayoutElement;

use std::mem;
use std::ffi::c_void;

pub trait VaoTrait {
    fn get_vao_id(&self) -> u32;

    fn bind_vao(&self) {
        unsafe {
            gl::BindVertexArray(self.get_vao_id());
        }
    }
}

pub trait VaoLayoutTrait: VaoTrait {
    fn get_layout_ref(&self) -> &Vec::<VaoLayoutElement>;

    fn get_mut_layout_ref(&mut self) -> &mut Vec::<VaoLayoutElement>;

    fn push_layout_element(&mut self, element_type: GLenum, normalized: GLboolean, element_count: GLint) {
        let layout: &mut Vec::<VaoLayoutElement> = self.get_mut_layout_ref();
        
        let mut nr_attributes = 1;
        unsafe {
            gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut nr_attributes);
        }
        
        if layout.len() >= nr_attributes as usize {
            println!("Error in push_layout_element. Exceeding maximum number of vertex attributes!!! Aborting.");
        } else {
            layout.push(VaoLayoutElement {
                used: true,
                element_type: element_type,
                normalized: normalized,
                element_count: element_count,
            })
        }
    }

    fn set_layout_element(&mut self, element_type: GLenum, normalized: GLboolean, element_count: GLint, attrb_array_number: usize) {
        let layout: &mut Vec::<VaoLayoutElement> = self.get_mut_layout_ref();
        
        let mut nr_attributes = 1;
        unsafe {
            gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut nr_attributes);
        }
        
        if attrb_array_number >= nr_attributes as usize {
            println!("Error in set_layout_element. Exceeding maximum number of vertex attributes!!! Aborting.");
        } else {
            if layout.len() < attrb_array_number + 1 {
                println!("Redemensioning Array!!!");

                for _ in layout.len()..attrb_array_number {
                    layout.push(VaoLayoutElement {
                        used: false,
                        element_type: 0,
                        normalized: gl::FALSE,
                        element_count: 0,
                    });
                }
            }

            layout.push(VaoLayoutElement {
                used: true,
                element_type: element_type,
                normalized: normalized,
                element_count: element_count,
            });

            println!("{}", layout.len());
        }
    }

    fn pop_layout_element(&mut self) {
        let layout: &mut Vec::<VaoLayoutElement> = self.get_mut_layout_ref();

        layout.pop();
    }

    fn clear_layout(&mut self) {
        let layout: &mut Vec::<VaoLayoutElement> = self.get_mut_layout_ref();

        *layout = Vec::<VaoLayoutElement>::new();
    }

    fn write_layout(&self) {
        let layout: &Vec::<VaoLayoutElement> = self.get_layout_ref();

        let mut stride: isize = 0;

        for i in layout {
            if i.used {
                stride += (i.element_count as usize * match i.element_type {
                    //I'll add more later.
                    //I should also make this a function...
                    gl::FLOAT   =>      mem::size_of::<GLfloat>(),
                    gl::INT     =>      mem::size_of::<GLint>(),
                    gl::UNSIGNED_INT => mem::size_of::<GLuint>(),
                    gl::BOOL    =>      mem::size_of::<GLboolean>(),
                    _ => { 
                        println!("Writing Layout Error!!! Unknown element type!!! Using GLuint.");
                        mem::size_of::<GLuint>()
                    }
                }) as isize;
            }
        }

        let mut offset = stride;
        for i in (0..layout.len()).rev() {
            if layout[i].used {
                unsafe {
                    offset -= layout[i].element_count as isize * (match layout[i].element_type {
                        //I'll add more later.
                        //I should also make this a function...
                        gl::FLOAT   =>      mem::size_of::<GLfloat>(),
                        gl::INT     =>      mem::size_of::<GLint>(),
                        gl::UNSIGNED_INT => mem::size_of::<GLuint>(),
                        gl::BOOL    =>      mem::size_of::<GLboolean>(),
                        _ => { 
                            println!("Writing Layout Error!!! Unknown element type!!! Using GLuint.");
                            mem::size_of::<GLuint>()
                        }
                    } as isize);

                    println!("Writing layout {}: {} elements of type {}, stride: {}, offset: {}", i, layout[i].element_count, layout[i].element_type, stride, offset);
                    self.bind_vao();
                    //self.vbo.bind();
                    gl::VertexAttribPointer(i as u32, layout[i].element_count, layout[i].element_type, layout[i].normalized, stride as i32, offset as *const c_void);
                    gl::EnableVertexAttribArray(i as u32);
                }
            }
        }
    }
}