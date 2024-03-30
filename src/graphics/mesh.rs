use std::ptr;

use super::{BufferObject, VertexAttribute, VAO};

pub struct Mesh {
    pub vao: VAO,
    pub vbo: BufferObject,
    pub ibo: Option<BufferObject>,
    pub idx_count: i32,
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

        Mesh {
            vao,
            vbo,
            ibo,
            idx_count,
        }
    }

    pub fn draw(&self) {
        self.vao.bind();
        unsafe {
            match &self.ibo {
                Some(_) => {
                    gl::DrawElements(gl::TRIANGLES, self.idx_count, gl::UNSIGNED_INT, ptr::null())
                }
                None => gl::DrawArrays(gl::TRIANGLES, 0, self.idx_count),
            }
        }
    }
}
