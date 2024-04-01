use std::cell::RefCell;
use std::ptr;

use gl::types::{GLsizei, GLuint};

use crate::{BufferObject, ShaderProgram, UniformType, VertexAttribute, VAO};

const DEFAULT_VERTEX_SHADER: &str = "../shaders/default_vertex.glsl";
const DEFAULT_FRAGMENT_SHADER: &str = "../shaders/default_fragment.glsl";

pub struct Mesh {
    pub vao: VAO,
    pub vbo: BufferObject,
    pub ibo: Option<BufferObject>,
    pub idx_count: GLsizei,
    pub shader: RefCell<ShaderProgram>,
    pub textures: Vec<GLuint>,
    pub material: RefCell<Option<Material>>,
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

        let shader = ShaderProgram::new(DEFAULT_VERTEX_SHADER, DEFAULT_FRAGMENT_SHADER);

        Mesh {
            vao,
            vbo,
            ibo,
            idx_count,
            shader: RefCell::new(shader),
            textures: Vec::new(),
            material: RefCell::new(None),
        }
    }

    pub fn set_shader(&self, shader: ShaderProgram) {
        let mut shader_ref = self.shader.borrow_mut();
        *shader_ref = shader;
        shader_ref.bind();
    }

    pub fn set_material(&self, material: Material) {
        self.material.borrow_mut().replace(material);
    }

    pub fn set_uniform<T: UniformType>(&self, uniform_name: &str, value: T) {
        let mut shader = self.shader.borrow_mut();
        shader.bind();
        shader.set_uniform(uniform_name, value);
    }

    pub fn set_vertex_attribute(&self, index: u32, size: i32, stride: i32, offset: Option<usize>) {
        self.vao.bind();
        VertexAttribute::create(
            index,
            size,
            gl::FLOAT,
            gl::FALSE,
            stride,
            offset
                .map(|x| (x * std::mem::size_of::<f32>()) as *const _)
                .unwrap_or(std::ptr::null()),
        );
    }

    pub fn set_color(&self, color: (f32, f32, f32, f32)) {
        self.set_uniform("u_Color", color);
    }

    pub fn draw(&self) {
        let mut shader = self.shader.borrow_mut();
        shader.bind();
        self.vao.bind();

        if let Some(material) = self.material.borrow().as_ref() {
            shader.set_uniform3f("material.ambient", material.ambient);
            shader.set_uniform3f("material.diffuse", material.diffuse);
            shader.set_uniform3f("material.specular", material.specular);
            shader.set_uniform1f("material.shininess", material.shininess);
        }

        for (i, &texture_id) in self.textures.iter().enumerate() {
            unsafe {
                gl::ActiveTexture(gl::TEXTURE0 + i as u32);
                gl::BindTexture(gl::TEXTURE_2D, texture_id);
            }
        }

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

pub struct Material {
    pub ambient: (f32, f32, f32),
    pub diffuse: (f32, f32, f32),
    pub specular: (f32, f32, f32),
    pub shininess: f32,
}
