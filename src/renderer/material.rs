/*  File: renderer/Material.rs
*   Author: Vicix
*
*   This file contains a class called Material.
*   Material holds a Shader and an array of textures. 
*   It makes easier to apply textures and shaders.
*/
extern crate gl;
use self::gl::types::*;

use crate::renderer::shader;
use crate::renderer::texture;
use crate::renderer::traits::shaderTrait::ShaderTrait;
use crate::renderer::traits::textureTrait::TextureTrait;

/*  This is the declaration of the class.
*   It holds an array of textures. 
*   For each texture we have a string (used for the uniform name) at the bind moment and a boolean value.
*   This value is true if the texture uniform has already been bound.
*/
pub struct Material {
    shader: shader::Shader,
    textures: Vec::<(texture::Texture, String, bool)>,
}

#[allow(dead_code)]
impl Material {
    /*  The constructor of the class.
    *   It takes a shader, makes a clone and creates a new textures vector.
    */
    pub fn new(shader: &shader::Shader) -> Material {
        Material {
            shader: shader.clone(),
            textures: Vec::<(texture::Texture, String, bool)>::new(),
        }
    }

    /*  This function is used to push a texture in the textures vector. 
    *   It also takes a uniform name, used in the binding.
    *   Note: the boolean value is always false, because the texture cannot be alreaby bound.
    */
    pub fn push_texture(&mut self, texture: &texture::Texture, uniform_name: &str) {
        self.textures.push((texture.clone(), uniform_name.to_string(), false));
    }

    /*  This function pops the pushed texture.
    */
    pub fn pop_texture(&mut self) {
        self.textures.pop();
    }

    /*  This function binds the shader and the textures.
    *   It also create uniforms for the to not bound textures.Material
    */
    pub fn bind(&mut self) {
        self.shader.bind();
        
        for (texture, uniform_name, bound) in &mut self.textures {
            texture.bind();
            if !*bound {
                texture.create_uniform_from_name(&self.shader, &uniform_name.as_str());
                *bound = true;
            }
        }
    }

    /*  This function is a link to the set_bool_uniform function in renderer::shader.
    *   We cannot access the shader in the Material so we must expose the function.
    */
    pub fn set_bool_uniform(&self, uniform_name: &str, uniform_value: bool) {
        self.shader.set_bool_uniform(uniform_name, uniform_value);
    }

    /*  This function is a link to the set_int_uniform function in renderer::shader.
    *   We cannot access the shader in the Material so we must expose the function.
    */
    pub fn set_int_uniform(&self, uniform_name: &str, uniform_value: i32) {
        self.shader.set_int_uniform(uniform_name, uniform_value);
    }

    /*  This function is a link to the set_float_uniform function in renderer::shader.
    *   We cannot access the shader in the Material so we must expose the function.
    */
    pub fn set_float_uniform(&self, uniform_name: &str, uniform_value: f32) {
        self.shader.set_float_uniform(uniform_name, uniform_value);
    }
}