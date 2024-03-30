use std::mem;
use std::os::raw::*;

use gl::types::{GLenum, GLsizeiptr, GLuint};

pub struct BufferObject {
    id: GLuint,
    r#type: GLenum,
    usage: GLenum,
}

impl BufferObject {
    pub fn new(r#type: GLenum, usage: GLenum) -> BufferObject {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }

        BufferObject { id, r#type, usage }
    }

    /// Create a new buffer object and bind it
    pub fn create(r#type: GLenum, usage: GLenum) -> BufferObject {
        let buffer = BufferObject::new(r#type, usage);
        buffer.bind();
        buffer
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.r#type, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(self.r#type, 0);
        }
    }

    pub fn store_f32_data(&self, data: &[f32]) {
        unsafe {
            gl::BufferData(
                self.r#type,
                (data.len() * mem::size_of::<f32>()) as GLsizeiptr,
                &data[0] as *const f32 as *const c_void,
                self.usage,
            )
        }
    }

    pub fn store_i32_data(&self, data: &[i32]) {
        unsafe {
            gl::BufferData(
                self.r#type,
                (data.len() * mem::size_of::<f32>()) as GLsizeiptr,
                &data[0] as *const i32 as *const c_void,
                self.usage,
            )
        }
    }

    pub fn store_u32_data(&self, data: &[u32]) {
        unsafe {
            gl::BufferData(
                self.r#type,
                (data.len() * mem::size_of::<f32>()) as GLsizeiptr,
                &data[0] as *const u32 as *const c_void,
                self.usage,
            )
        }
    }
}
