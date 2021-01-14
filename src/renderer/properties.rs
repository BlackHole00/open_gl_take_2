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

/*  This struct contains the opengl properties.
*   ***I should find a more modular system***
*/
#[derive(Debug, Clone)]
pub struct TextureGlProperties {
    pub active_texture_number: GLuint,
    pub texture_type: GLenum,
    pub texture_warp_s: GLenum,
    pub texture_warp_t: GLenum,
    pub texture_min_filter: GLenum,
    pub texture_mag_filter: GLenum,
    pub texture_uniform_name: String,
}

/*  This struct contains the image properties.
*   ***I should find a more modular system***
*/
#[derive(Debug, Clone)]
pub struct TextureImageProperties {
    pub path: String,
    pub fliph: bool,
    pub flipv: bool,
    pub internal_format: GLenum,
    pub format: GLenum,
}