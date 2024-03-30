use std::cell::RefCell;
use std::ptr;

use crate::{BufferObject, ShaderProgram, VertexAttribute, VAO};

const DEFAULT_VERTEX_SHADER: &str = "../shaders/default_vertex.glsl";
const DEFAULT_FRAGMENT_SHADER: &str = "../shaders/default_fragment.glsl";

pub struct Mesh {
    pub vao: VAO,
    pub vbo: BufferObject,
    pub ibo: Option<BufferObject>,
    pub idx_count: i32,
    pub shader: RefCell<ShaderProgram>,
    pub color: RefCell<(f32, f32, f32, f32)>,
}

impl Mesh {
    pub fn new(vertices: &[f32], indices: Option<&[u32]>) -> Mesh {
        let vao = VAO::create();

        let vbo = BufferObject::create(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
        vbo.store_f32_data(vertices);

        let ibo = indices.map(|inds| {
            let ibo = BufferObject::create(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
            ibo.store_u32_data(inds);
            ibo
        });

        VertexAttribute::create_default(0, 3);

        let idx_count = indices.map_or(vertices.len() as i32 / 3, |inds| inds.len() as i32);

        let mut shader = ShaderProgram::new(DEFAULT_VERTEX_SHADER, DEFAULT_FRAGMENT_SHADER);
        shader.create_uniform("u_Color");

        Mesh {
            vao,
            vbo,
            ibo,
            idx_count,
            shader: RefCell::new(shader),
            color: RefCell::new((1.0, 1.0, 1.0, 1.0)),
        }
    }

    pub fn set_shader(&self, shader: ShaderProgram) {
        *self.shader.borrow_mut() = shader;
    }

    pub fn set_color(&self, color: (f32, f32, f32, f32)) {
        *self.color.borrow_mut() = color;
    }

    pub fn draw(&self) {
        let mut shader = self.shader.borrow_mut();
        shader.bind();
        shader.set_uniform4f("u_Color", self.color.borrow().clone());

        self.vao.bind();
        unsafe {
            match &self.ibo {
                Some(_) => {
                    gl::DrawElements(gl::TRIANGLES, self.idx_count, gl::UNSIGNED_INT, ptr::null())
                }
                None => gl::DrawArrays(gl::TRIANGLES, 0, self.idx_count),
            }
        }

        shader.unbind();
    }
}
