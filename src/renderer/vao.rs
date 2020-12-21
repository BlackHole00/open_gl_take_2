extern crate gl;
use self::gl::types::*;

use std::ffi::c_void;
use std::mem;

#[derive(Clone)]
pub struct Vao {
    vao_id: GLuint,
    layout: Vec::<VaoLayoutElement>,
}

#[allow(dead_code)]
impl Vao{
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

    pub fn push_layout_element(&mut self, element_type: GLenum, normalized: GLboolean, element_count: GLint) {
        let mut nr_attributes = 1;
        unsafe {
            gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut nr_attributes);
        }
        
        if self.layout.len() >= nr_attributes as usize {
            println!("Error in push_layout_element. Exceeding maximum number of vertex attributes!!! Aborting.");
        } else {
            self.layout.push(VaoLayoutElement {
                used: true,
                element_type: element_type,
                normalized: normalized,
                element_count: element_count,
            })
        }
    }

    pub fn set_layout_element(&mut self, element_type: GLenum, normalized: GLboolean, element_count: GLint, attrb_array_number: usize) {
        let mut nr_attributes = 1;
        unsafe {
            gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut nr_attributes);
        }
        
        if attrb_array_number >= nr_attributes as usize {
            println!("Error in set_layout_element. Exceeding maximum number of vertex attributes!!! Aborting.");
        } else {
            if self.layout.len() < attrb_array_number + 1 {
                println!("Redemensioning Array!!!");

                for _ in self.layout.len()..attrb_array_number {
                    self.layout.push(VaoLayoutElement {
                        used: false,
                        element_type: 0,
                        normalized: gl::FALSE,
                        element_count: 0,
                    });
                }
            }

            self.layout.push(VaoLayoutElement {
                used: true,
                element_type: element_type,
                normalized: normalized,
                element_count: element_count,
            });

            println!("{}", self.layout.len());
        }
    }

    pub fn pop_layout_element(&mut self) {
        self.layout.pop().unwrap();
    }

    pub fn clear_layout(&mut self) {
        self.layout = Vec::<VaoLayoutElement>::new();
    }

    pub fn write_layout(&self) {
        let mut stride: isize = 0;

        for i in &self.layout {
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
        for i in (0..self.layout.len()).rev() {
            if self.layout[i].used {
                unsafe {
                    offset -= self.layout[i].element_count as isize * (match self.layout[i].element_type {
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

                    println!("Writing layout {}: {} elements of type {}, stride: {}, offset: {} {}", i, self.layout[i].element_count, self.layout[i].element_type, stride, offset, mem::size_of::<GLfloat>());
                    self.bind();
                    //self.vbo.bind();
                    gl::VertexAttribPointer(i as u32, self.layout[i].element_count, self.layout[i].element_type, self.layout[i].normalized, stride as i32, offset as *const c_void);
                    gl::EnableVertexAttribArray(i as u32);
                }
            }
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vao_id);
        }
    }
}

#[derive(Clone)]
struct VaoLayoutElement {
    used: bool,
    element_type: GLenum,
    normalized: GLboolean,
    element_count: GLint,
}