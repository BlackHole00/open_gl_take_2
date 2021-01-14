use std::ffi::CString;

pub trait ShaderTrait {
    fn get_program_id(&self) -> u32;

    /*  This is a simple function that binds the shader
    */
    fn bind(&self) {
        unsafe {
            gl::UseProgram(self.get_program_id());
        }
    }

    /*  This is a simple function that takes an uniform name and vaue and makes a boolean uniform.
    */
    fn set_bool_uniform(&self, uniform_name: &str, uniform_value: bool) {
        unsafe {
            self.bind();
            let uniform_location = gl::GetUniformLocation(self.get_program_id(), CString::new(uniform_name.as_bytes()).unwrap().as_ptr());
            gl::Uniform1i(uniform_location, uniform_value as i32);
        }
    }

    /*  This is a simple function that takes an uniform name and vaue and makes an integer uniform.
    */
    fn set_int_uniform(&self, uniform_name: &str, uniform_value: i32) {
        unsafe {
            self.bind();
            let uniform_location = gl::GetUniformLocation(self.get_program_id(), CString::new(uniform_name.as_bytes()).unwrap().as_ptr());
            gl::Uniform1i(uniform_location, uniform_value);
        }
    }

    /*  This is a simple function that takes an uniform name and vaue and makes a float uniform.
    */
    fn set_float_uniform(&self, uniform_name: &str, uniform_value: f32) {
        unsafe {
            self.bind();
            let uniform_location = gl::GetUniformLocation(self.get_program_id(), CString::new(uniform_name.as_bytes()).unwrap().as_ptr());
            gl::Uniform1f(uniform_location, uniform_value);
        }
    }

    /*  This is a simple function that takes an uniform name and vaue and makes a vec3 uniform.
    */
    fn set_3float_uniform(&self, uniform_name: &str, uniform_value1: f32, uniform_value2: f32, uniform_value3: f32) {
        unsafe {
            self.bind();
            let uniform_location = gl::GetUniformLocation(self.get_program_id(), CString::new(uniform_name.as_bytes()).unwrap().as_ptr());
            gl::Uniform3f(uniform_location, uniform_value1, uniform_value2, uniform_value3);
        }
    }
}