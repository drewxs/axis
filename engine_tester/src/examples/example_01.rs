#![allow(dead_code)]

use std::{mem, ptr};

use gl::types::{GLfloat, GLsizei};

use axis::{
    gl_wrapper::{BufferObject, VertexAttribute, VAO},
    window::Window,
};

pub fn draw_rect() {
    let mut window = Window::new(1080, 720, "Window");
    window.init_gl();

    let vertices = [
        0.5, 0.5, 0.0, // vertex 1
        0.5, -0.5, 0.0, // vertex 2
        -0.5, -0.5, 0.0, // vertex 3
        -0.5, 0.5, 0.0, // vertex 4
    ];
    let indices = [
        0, 1, 3, // 1st triangle
        1, 2, 3, // 2nd triangle
    ];

    let vao = VAO::new();
    vao.bind();

    let vbo = BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.bind();
    vbo.store_f32_data(&vertices);

    let ibo = BufferObject::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
    ibo.bind();
    ibo.store_i32_data(&indices);

    let position_attribute = VertexAttribute::new(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        3 * mem::size_of::<GLfloat>() as GLsizei,
        ptr::null(),
    );

    let index_attribute = VertexAttribute::new(
        1,
        3,
        gl::FLOAT,
        gl::FALSE,
        3 * mem::size_of::<GLfloat>() as GLsizei,
        ptr::null(),
    );

    position_attribute.enable();
    index_attribute.enable();

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.3, 0.5, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null())
        }
        window.update();
    }
}

pub fn draw_triangle() {
    let mut window = Window::new(1080, 720, "Triangle");
    window.init_gl();

    let vertices = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

    let vao = VAO::new();
    vao.bind();

    let vbo = BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.bind();
    vbo.store_f32_data(&vertices);

    let position_attribute = VertexAttribute::new(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        3 * mem::size_of::<GLfloat>() as GLsizei,
        ptr::null(),
    );
    position_attribute.enable();

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.3, 0.5, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawArrays(gl::TRIANGLES, 0, 3)
        }
        window.update();
    }
}
