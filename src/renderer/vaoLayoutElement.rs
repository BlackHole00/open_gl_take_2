/*  This Is the VaoLayoutElement class.
*   The used flag is true if this Layout Element is used.
*   The element type is the opengl element type
*   The normalized flag is true if the value is normalized.
*   The element count is how many element there are.
*   ***what a description!***
*/
extern crate gl;
use self::gl::types::*;

#[derive(Clone)]
pub struct VaoLayoutElement {
    pub used: bool,
    pub element_type: GLenum,
    pub normalized: GLboolean,
    pub element_count: GLint,
}