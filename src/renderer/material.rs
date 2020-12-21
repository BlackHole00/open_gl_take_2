use crate::renderer::shader;
use crate::renderer::texture;

pub struct Material {
    shader: shader::Shader,
    textures: Vec::<(texture::Texture, String, bool)>,
}

#[allow(dead_code)]
impl Material {
    pub fn new(shader: &shader::Shader) -> Material {
        Material {
            shader: shader.clone(),
            textures: Vec::<(texture::Texture, String, bool)>::new(),
        }
    }

    pub fn push_texture(&mut self, texture: &texture::Texture, uniform_name: &str) {
        self.textures.push((texture.clone(), uniform_name.to_string(), false));
    }

    pub fn pop_texture(&mut self) {
        self.textures.pop();
    }

    pub fn bind(&mut self) {
        self.shader.bind();
        
        for (texture, uniform_name, binded) in &mut self.textures {
            texture.bind();
            if !*binded {
                texture.create_uniform_from_name(&self.shader, &uniform_name.as_str());
                *binded = true;
            }
        }
    }

    pub fn set_bool_uniform(&self, uniform_name: &str, uniform_value: bool) {
        self.shader.set_bool_uniform(uniform_name, uniform_value);
    }

    pub fn set_int_uniform(&self, uniform_name: &str, uniform_value: i32) {
        self.shader.set_int_uniform(uniform_name, uniform_value);
    }

    pub fn set_float_uniform(&self, uniform_name: &str, uniform_value: f32) {
        self.shader.set_float_uniform(uniform_name, uniform_value);
    }
}