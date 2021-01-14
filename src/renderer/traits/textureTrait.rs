use crate::renderer::properties::{TextureGlProperties, TextureImageProperties};
use crate::renderer::traits::shaderTrait::ShaderTrait;
use crate::renderer::constants;

extern crate image;
use image::GenericImage;

extern crate gl;
use self::gl::types::*;

use std::ffi::c_void;

pub trait TextureTrait {
    fn get_texture_id(&self) -> u32;
    fn get_gl_properties_ref(&self) -> &TextureGlProperties;
    fn get_mut_gl_properties_ref(&mut self) -> &mut TextureGlProperties;
    fn get_image_properties_ref(&self) -> &TextureImageProperties;
    fn get_mut_image_properties_ref(&mut self) -> &mut TextureImageProperties;

    /*  This function is used to set the texture parameters, to open the image from the path and to give the texture to opengl
    *   It also checks if the texture should be flipped.
    */
    fn gen_texture(&self) {
        let gl_properties = self.get_gl_properties_ref();
        let image_properties = self.get_image_properties_ref();

        unsafe {
            gl::BindTexture(gl_properties.texture_type, self.get_texture_id());

            gl::TexParameteri(gl_properties.texture_type, gl::TEXTURE_WRAP_S, gl_properties.texture_warp_s as i32);
            gl::TexParameteri(gl_properties.texture_type, gl::TEXTURE_WRAP_T, gl_properties.texture_warp_t as i32);
            gl::TexParameteri(gl_properties.texture_type, gl::TEXTURE_MIN_FILTER, gl_properties.texture_min_filter as i32);
            gl::TexParameteri(gl_properties.texture_type, gl::TEXTURE_MAG_FILTER, gl_properties.texture_mag_filter as i32);

            let mut img = image::open(&image_properties.path).expect("Failed to open the texture {}");
            if image_properties.fliph {
                img = img.fliph();
            }
            if image_properties.flipv {
                img = img.flipv();
            }
            let data = img.raw_pixels();
            gl::TexImage2D(gl::TEXTURE_2D, 0, image_properties.internal_format as i32, img.width() as i32, img.height() as i32, 0, image_properties.format, gl::UNSIGNED_BYTE, &data[0] as *const u8 as  *const c_void);
        }
    }

    /*  This function is used to set a opengl property in the gl_properties struct inside the class.
    *   It takes the property_id that is a constant defined in renderer::constants and the value to set.
    *   It matches the property_id and sets the right property.
    *   ***I should really find a more dynamic way to set the property maybe using an array... idk...***
    */
    fn set_gl_property(&mut self, property: GLenum, value: GLenum) {
        let texture_id = self.get_texture_id();
        let gl_properties = self.get_mut_gl_properties_ref();
        match property {
            gl::TEXTURE_WRAP_S => gl_properties.texture_warp_s = value,
            gl::TEXTURE_WRAP_T => gl_properties.texture_warp_t = value,
            gl::TEXTURE_MIN_FILTER => gl_properties.texture_min_filter = value,
            gl::TEXTURE_MAG_FILTER => gl_properties.texture_mag_filter = value,
            _ => println!("Wrong gl property set on texture {} (id: {})", gl_properties.texture_uniform_name, texture_id),
        }
    }

    /*  This is a simple function used to set the type of texture.
    */
    fn set_texture_type(&mut self, texture_type: GLenum) {
        self.get_mut_gl_properties_ref().texture_type = texture_type;
    }

    /*  This is a simple function used to set the active texture number.
    */
    fn set_active_texture_number(&mut self, active_texture_number: GLuint) {
        self.get_mut_gl_properties_ref().active_texture_number = active_texture_number;
    }

    /*  This is a simple function used to set the uniform name.
    */
    fn set_uniform_name(&mut self, uniform_name: &str) {
        self.get_mut_gl_properties_ref().texture_uniform_name = uniform_name.to_string();
    }

    /*  This is a simple function used create an uniform in the shader using a uniform_name.
    *   This function is basically the set_uniform_name and create_uniform functions conbined.
    */
    fn create_uniform_from_name(&mut self, shader: &dyn ShaderTrait, uniform_name: &str) {
        self.set_uniform_name(uniform_name);
        self.create_uniform(shader);
    }
 
    /*  This is a simple function used to set a uniform.
    */
    fn create_uniform(&self, shader: &dyn ShaderTrait) {
        shader.set_int_uniform(self.get_gl_properties_ref().texture_uniform_name.as_str(), self.get_gl_properties_ref().active_texture_number as i32)
    }

    /*  This is a simple function used to generate the mipmap.
    */
    fn generate_mipmap(&self) {
        unsafe {
            gl::GenerateMipmap(self.get_gl_properties_ref().texture_type);
        }
    }

    /*  This function is used to set a image property in the image_properties struct inside the class.
    *   It takes the property_id that is a constant defined in renderer::constants and the value to set.
    *   It matches the property_id and sets the right property.
    *   ***I should really find a more dynamic way to set the property maybe using an array... idk...***
    */
    fn set_image_property(&mut self, property_id: u8, property: GLenum) {
        let texture_id = self.get_texture_id();
        let image_properties = self.get_mut_image_properties_ref();

        match property_id {
            constants::FLIP_H_PROPERTY => image_properties.fliph = property != 0, //weird cast to bool because rust
            constants::FLIP_V_PROPERTY => image_properties.flipv = property != 0,
            constants::INTERNAL_FORMAT_PROPERTY => image_properties.internal_format = property,
            constants::FORMAT_PROPERTY => image_properties.format = property,
            _ => println!("Wrong image property set on texture id: {}", texture_id),
        }
    }

    /*  This is a simple function used to bind the texture to the right active texture.
    */
    fn bind(&self) {
        let texture_id = self.get_texture_id();
        let gl_properties = self.get_gl_properties_ref();

        unsafe {
            match gl_properties.active_texture_number {
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
                    println!("Invalid active texture number in texture {} (id: {}). Using Active Texture 15.", gl_properties.texture_uniform_name, texture_id);
                    gl::ActiveTexture(gl::TEXTURE15);
                },
            }
            gl::BindTexture(gl_properties.texture_type, texture_id);
        }
    }
}