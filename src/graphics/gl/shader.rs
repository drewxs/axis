use std::collections::HashMap;
use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::ptr;

use cgmath::Matrix;
use gl::types::GLchar;

pub struct ShaderProgram {
    program_handle: u32,
    uniform_ids: HashMap<String, i32>,
}

impl ShaderProgram {
    pub fn new(vertex_shader_path: &str, fragment_shader_path: &str) -> ShaderProgram {
        let mut vertex_shader_file = File::open(vertex_shader_path)
            .unwrap_or_else(|_| panic!("Failed to open {vertex_shader_path}"));
        let mut fragment_shader_file = File::open(fragment_shader_path)
            .unwrap_or_else(|_| panic!("Failed to open {fragment_shader_path}"));

        let mut vertex_shader_source = String::new();
        let mut fragment_shader_source = String::new();

        vertex_shader_file
            .read_to_string(&mut vertex_shader_source)
            .expect("Failed to read vertex shader");
        fragment_shader_file
            .read_to_string(&mut fragment_shader_source)
            .expect("Failed to read vertex shader");

        unsafe {
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            let c_str_vert = CString::new(vertex_shader_source.as_bytes()).unwrap();
            gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), std::ptr::null());
            gl::CompileShader(vertex_shader);

            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            let c_str_vert = CString::new(fragment_shader_source.as_bytes()).unwrap();
            gl::ShaderSource(fragment_shader, 1, &c_str_vert.as_ptr(), std::ptr::null());
            gl::CompileShader(fragment_shader);

            let program_handle = gl::CreateProgram();
            gl::AttachShader(program_handle, vertex_shader);
            gl::AttachShader(program_handle, fragment_shader);
            gl::LinkProgram(program_handle);

            let mut success = 0;
            gl::GetProgramiv(program_handle, gl::LINK_STATUS, &mut success);
            if success == 0 {
                // Obtain and log the link error
                let mut info_log = vec![0; 512];
                gl::GetProgramInfoLog(
                    program_handle,
                    512,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                println!(
                    "ERROR::SHADER::PROGRAM::LINKING_FAILED\n{}",
                    std::str::from_utf8(&info_log).unwrap()
                );
                panic!("Failed to link shader program");
            }

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            ShaderProgram {
                program_handle,
                uniform_ids: HashMap::new(),
            }
        }
    }

    pub fn create(vertex_shader_path: &str, fragment_shader_path: &str) -> ShaderProgram {
        let program = ShaderProgram::new(vertex_shader_path, fragment_shader_path);
        program.bind();
        program
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.program_handle);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }

    pub fn create_uniform(&mut self, uniform_name: &str) {
        self.bind();

        let name = CString::new(uniform_name).unwrap();
        let uniform_location =
            unsafe { gl::GetUniformLocation(self.program_handle, name.as_ptr()) };

        if uniform_location == -1 {
            panic!("Cannot locate uniform: {uniform_name}");
        } else {
            self.uniform_ids
                .insert(uniform_name.to_string(), uniform_location);
        }
    }

    pub fn set_uniform4f(&mut self, uniform_name: &str, value: (f32, f32, f32, f32)) {
        self.bind();

        let uniform_location = *self
            .uniform_ids
            .entry(uniform_name.to_string())
            .or_insert_with(|| {
                let name = CString::new(uniform_name).unwrap();
                unsafe { gl::GetUniformLocation(self.program_handle, name.as_ptr()) }
            });

        unsafe {
            let (r, g, b, a) = value;
            gl::Uniform4f(uniform_location, r, g, b, a);
        }
    }

    pub fn set_matrix4fv_uniform(&self, uniform_name: &str, matrix: &cgmath::Matrix4<f32>) {
        unsafe {
            gl::UniformMatrix4fv(
                self.uniform_ids[uniform_name],
                1,
                gl::FALSE,
                matrix.as_ptr(),
            )
        }
    }
}
