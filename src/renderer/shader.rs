extern crate gl;
use self::gl::types::*;

use std::ffi::CString;
use std::ptr;
use std::str;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Clone)]
pub struct Shader {
    program_id: GLuint,
}

#[allow(dead_code)]
impl Shader {
    pub fn new(vertex_path: &str, fragment_path: &str) -> Shader {
        let mut v_shader_file = File::open(vertex_path)
            .unwrap_or_else(|_| panic!("Failed to open {}", vertex_path));
        let mut f_shader_file = File::open(fragment_path)
            .unwrap_or_else(|_| panic!("Failed to open {}", fragment_path));
        let mut vertex_code = String::new();
        let mut fragment_code = String::new();
        v_shader_file
            .read_to_string(&mut vertex_code)
            .expect("Failed to read vertex shader");
        f_shader_file
            .read_to_string(&mut fragment_code)
            .expect("Failed to read fragment shader");

        let shader_program = unsafe {
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            let c_str_source = CString::new(vertex_code.as_bytes()).unwrap();
            gl::ShaderSource(vertex_shader, 1, &c_str_source.as_ptr(), ptr::null());
            gl::CompileShader(vertex_shader);

            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            let c_str_source = CString::new(fragment_code.as_bytes()).unwrap();
            gl::ShaderSource(fragment_shader, 1, &c_str_source.as_ptr(), ptr::null());
            gl::CompileShader(fragment_shader);

            let shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);

            let mut success = 0;
            let mut info_log: Vec::<u8> = Vec::with_capacity(512);
            info_log.set_len(512 - 1);
            gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetProgramInfoLog(shader_program, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!("ERROR::SHADER::PROGRAM::LINK_FAILED\n{}", str::from_utf8(&info_log).unwrap());
            } else {
                println!("SHADERS SUCCESSFULLY COMPILED AND LINKED");
            }

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            shader_program
        };

        Shader {
            program_id: shader_program,
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.program_id);
        }
    }

    pub fn set_bool_uniform(&self, uniform_name: &str, uniform_value: bool) {
        unsafe {
            self.bind();
            let uniform_location = gl::GetUniformLocation(self.program_id, CString::new(uniform_name.as_bytes()).unwrap().as_ptr());
            gl::Uniform1i(uniform_location, uniform_value as i32);
        }
    }

    pub fn set_int_uniform(&self, uniform_name: &str, uniform_value: i32) {
        unsafe {
            self.bind();
            let uniform_location = gl::GetUniformLocation(self.program_id, CString::new(uniform_name.as_bytes()).unwrap().as_ptr());
            gl::Uniform1i(uniform_location, uniform_value);
        }
    }

    pub fn set_float_uniform(&self, uniform_name: &str, uniform_value: f32) {
        unsafe {
            self.bind();
            let uniform_location = gl::GetUniformLocation(self.program_id, CString::new(uniform_name.as_bytes()).unwrap().as_ptr());
            gl::Uniform1f(uniform_location, uniform_value);
        }
    }
}