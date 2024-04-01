use std::mem;
use std::os::raw::*;

use gl::types::{GLboolean, GLenum, GLfloat, GLint, GLsizei, GLuint};

/// Vertex Array Object
pub struct VAO {
    id: GLuint,
}

impl VAO {
    pub fn new() -> VAO {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }

        VAO { id }
    }

    /// Create a new VAO and bind it
    pub fn create() -> VAO {
        let vao = VAO::new();
        vao.bind();
        vao
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

pub struct VertexAttribute {
    index: GLuint,
}

impl VertexAttribute {
    pub fn new(
        index: GLuint,
        size: GLint,
        r#type: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        pointer: *const c_void,
    ) -> VertexAttribute {
        unsafe {
            gl::VertexAttribPointer(index, size, r#type, normalized, stride, pointer);
        }

        VertexAttribute { index }
    }

    /// Create a new vertex attribute and enable it
    pub fn create(
        index: GLuint,
        size: GLint,
        r#type: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        pointer: *const c_void,
    ) -> VertexAttribute {
        let va = VertexAttribute::new(index, size, r#type, normalized, stride, pointer);
        va.enable();
        va
    }

    /// Create a new vertex attribute with default values
    pub fn default(index: GLuint, size: GLint) -> VertexAttribute {
        VertexAttribute::new(
            index,
            size,
            gl::FLOAT,
            gl::FALSE,
            3 * mem::size_of::<GLfloat>() as GLsizei,
            std::ptr::null(),
        )
    }

    /// Create a new vertex attribute with default values and enable it
    pub fn create_default(index: GLuint, size: GLint) -> VertexAttribute {
        let va = VertexAttribute::default(index, size);
        va.enable();
        va
    }

    pub fn enable(&self) {
        unsafe {
            gl::EnableVertexAttribArray(self.index);
        }
    }

    pub fn disable(&self) {
        unsafe {
            gl::DisableVertexAttribArray(self.index);
        }
    }
}
