/*  This struct contains the properties that are used to draw.
*   ***I should find a more modular system***
*/
extern crate gl;
use self::gl::types::*;

#[derive(Clone)]
pub struct GlObjectProperties {
    pub draw_mode: GLenum,
    pub ebo_type: GLenum,
}