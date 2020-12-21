extern crate gl;
use self::gl::types::*;

use std::os::raw::c_void;

extern crate image;
use image::GenericImage;

use crate::renderer::shader::Shader;
use crate::renderer::constants;

#[derive(Debug, Clone)]
pub struct Texture {
    texture_id: GLuint,
    gl_properties: GlProperties,
    image_properties: ImageProperties,
}

#[allow(dead_code)]
impl Texture {
    pub fn new(texture_type: GLenum, image_path: &str, internal_format: GLenum, format: GLenum, active_texture_number: GLuint) -> Texture {
        let texture_id = unsafe {
            let mut texture_id = 1;
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            texture_id
        };

        Texture {
            texture_id: texture_id,
            gl_properties: GlProperties {
                active_texture_number: active_texture_number,
                texture_type: texture_type,
                texture_warp_s: gl::MIRRORED_REPEAT,
                texture_warp_t: gl::MIRRORED_REPEAT,
                texture_min_filter: gl::NEAREST,
                texture_mag_filter: gl::NEAREST,
                texture_uniform_name: image_path.to_string(),
            },
            image_properties: ImageProperties {
                path: image_path.to_string(),
                fliph: false,
                flipv: false,
                internal_format: internal_format,
                format: format,
            },
        }
    }

    pub fn gen_texture(&self) {
        unsafe {
            gl::BindTexture(self.gl_properties.texture_type, self.texture_id);

            gl::TexParameteri(self.gl_properties.texture_type, gl::TEXTURE_WRAP_S, self.gl_properties.texture_warp_s as i32);
            gl::TexParameteri(self.gl_properties.texture_type, gl::TEXTURE_WRAP_T, self.gl_properties.texture_warp_t as i32);
            gl::TexParameteri(self.gl_properties.texture_type, gl::TEXTURE_MIN_FILTER, self.gl_properties.texture_min_filter as i32);
            gl::TexParameteri(self.gl_properties.texture_type, gl::TEXTURE_MAG_FILTER, self.gl_properties.texture_mag_filter as i32);

            let mut img = image::open(&self.image_properties.path).expect("Failed to open the texture {}");
            if self.image_properties.fliph {
                img = img.fliph();
            }
            if self.image_properties.flipv {
                img = img.flipv();
            }
            let data = img.raw_pixels();
            gl::TexImage2D(gl::TEXTURE_2D, 0, self.image_properties.internal_format as i32, img.width() as i32, img.height() as i32, 0, self.image_properties.format, gl::UNSIGNED_BYTE, &data[0] as *const u8 as  *const c_void);
        }
    }

    pub fn set_gl_property(&mut self, property: GLenum, value: GLenum) {
        //Awful and wrong way do do this... I'll change this later.
        match property {
            gl::TEXTURE_WRAP_S => self.gl_properties.texture_warp_s = value,
            gl::TEXTURE_WRAP_T => self.gl_properties.texture_warp_t = value,
            gl::TEXTURE_MIN_FILTER => self.gl_properties.texture_min_filter = value,
            gl::TEXTURE_MAG_FILTER => self.gl_properties.texture_mag_filter = value,
            _ => println!("Wrong gl property set on texture {} (id: {})", self.gl_properties.texture_uniform_name, self.texture_id),
        }
    }

    pub fn set_texture_type(&mut self, texture_type: GLenum) {
        self.gl_properties.texture_type = texture_type;
    }

    pub fn set_active_texture_number(&mut self, active_texture_number: GLuint) {
        self.gl_properties.active_texture_number = active_texture_number;
    }
    
    pub fn set_uniform_name(&mut self, uniform_name: &str) {
        self.gl_properties.texture_uniform_name = uniform_name.to_string();
    }

    pub fn create_uniform_from_name(&mut self, shader: &Shader, uniform_name: &str) {
        self.set_uniform_name(uniform_name);
        self.create_uniform(shader);
    }

    pub fn create_uniform(&self, shader: &Shader) {
        shader.set_int_uniform(self.gl_properties.texture_uniform_name.as_str(), self.gl_properties.active_texture_number as i32)
    }

    pub fn generate_mipmap(&self) {
        unsafe {
            gl::GenerateMipmap(self.gl_properties.texture_type);
        }
    }

    pub fn set_image_property(&mut self, property_id: u8, property: GLenum) {
        match property_id {
            constants::FLIP_H_PROPERTY => self.image_properties.fliph = property != 0, //weird cast to bool because rust
            constants::FLIP_V_PROPERTY => self.image_properties.flipv = property != 0,
            constants::INTERNAL_FORMAT_PROPERTY => self.image_properties.internal_format = property,
            constants::FORMAT_PROPERTY => self.image_properties.format = property,
            _ => println!("Wrong image property set on texture {} (id: {})", self.gl_properties.texture_uniform_name, self.texture_id),
        }
    }

    pub fn bind(&self) {
        unsafe {
            match self.gl_properties.active_texture_number {
                0 => gl::ActiveTexture(gl::TEXTURE0),
                1 => gl::ActiveTexture(gl::TEXTURE1),
                2 => gl::ActiveTexture(gl::TEXTURE2),
                3 => gl::ActiveTexture(gl::TEXTURE3),
                4 => gl::ActiveTexture(gl::TEXTURE4),
                5 => gl::ActiveTexture(gl::TEXTURE5),
                6 => gl::ActiveTexture(gl::TEXTURE6),
                7 => gl::ActiveTexture(gl::TEXTURE7),
                8 => gl::ActiveTexture(gl::TEXTURE8),
                9 => gl::ActiveTexture(gl::TEXTURE9),
                10 => gl::ActiveTexture(gl::TEXTURE10),
                11 => gl::ActiveTexture(gl::TEXTURE11),
                12 => gl::ActiveTexture(gl::TEXTURE12),
                13 => gl::ActiveTexture(gl::TEXTURE13),
                14 => gl::ActiveTexture(gl::TEXTURE14),
                15 => gl::ActiveTexture(gl::TEXTURE15),
                _ => {
                    println!("Invalid active texture number in texture {} (id: {}). Using Active Texture 15.", self.gl_properties.texture_uniform_name, self.texture_id);
                    gl::ActiveTexture(gl::TEXTURE15);
                },
            }
            gl::BindTexture(self.gl_properties.texture_type, self.texture_id);
        }
    }
}

#[derive(Debug, Clone)]
struct GlProperties {
    active_texture_number: GLuint,
    texture_type: GLenum,
    texture_warp_s: GLenum,
    texture_warp_t: GLenum,
    texture_min_filter: GLenum,
    texture_mag_filter: GLenum,
    texture_uniform_name: String,
}

#[derive(Debug, Clone)]
struct ImageProperties {
    path: String,
    fliph: bool,
    flipv: bool,
    internal_format: GLenum,
    format: GLenum,
}