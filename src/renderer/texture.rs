/*  File: renderer/texture.rs
*   Author: Vicix
*
*   This file contains the Texture class.
*   The class Shader is a simple abstaction of the opengl texture. 
*   It helps creating textures from files and to create uniforms.
*/

extern crate gl;
use self::gl::types::*;

use crate::renderer::properties::{TextureGlProperties, TextureImageProperties};
use crate::renderer::traits::textureTrait::TextureTrait;

/*  This is the declaration on the class.
*   It contains the texture_id (the only this that is strictly necessary).
*   It also contains the opengl texture properties an the image properties.
*/
#[derive(Debug, Clone)]
pub struct Texture {
    texture_id: GLuint,
    gl_properties: TextureGlProperties,
    image_properties: TextureImageProperties,
}

#[allow(dead_code)]
impl Texture {
    /*  This is the constructor of the class.
    *   It takes the texture type, the image path the internal format, the normal format and the active texture number.
    *   It also create an opengl texture.
    *   Note: The default uniform name is set to the image path.
    */
    pub fn new(texture_type: GLenum, image_path: &str, internal_format: GLenum, format: GLenum, active_texture_number: GLuint) -> Texture {
        let texture_id = unsafe {
            let mut texture_id = 1;
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            texture_id
        };

        Texture {
            texture_id: texture_id,
            gl_properties: TextureGlProperties {
                active_texture_number: active_texture_number,
                texture_type: texture_type,
                texture_warp_s: gl::MIRRORED_REPEAT,
                texture_warp_t: gl::MIRRORED_REPEAT,
                texture_min_filter: gl::NEAREST,
                texture_mag_filter: gl::NEAREST,
                texture_uniform_name: image_path.to_string(),
            },
            image_properties: TextureImageProperties {
                path: image_path.to_string(),
                fliph: false,
                flipv: false,
                internal_format: internal_format,
                format: format,
            },
        }
    }
}

impl TextureTrait for Texture {
    fn get_texture_id(&self) -> u32 {
        self.texture_id
    }

    fn get_gl_properties_ref(&self) -> &TextureGlProperties {
        &self.gl_properties
    }

    fn get_mut_gl_properties_ref(&mut self) -> &mut TextureGlProperties {
        &mut self.gl_properties
    }

    fn get_image_properties_ref(&self) -> &TextureImageProperties {
        &self.image_properties
    }

    fn get_mut_image_properties_ref(&mut self) -> &mut TextureImageProperties {
        &mut self.image_properties
    }
}
