#![allow(dead_code)]

use std::ptr;

use axis::{
    gl_wrapper::{BufferObject, VertexAttribute, VAO},
    window::Window,
};

pub fn draw_rect() {
    let mut window = Window::create(1080, 720, "Window");

    let vertices = [
        -0.1, 0.5, 0.0, // vertex 1
        -0.1, -0.5, 0.0, // vertex 2
        -0.9, -0.5, 0.0, // vertex 3
        -0.9, 0.5, 0.0, // vertex 4
    ];
    let indices = [
        0, 1, 3, // 1st triangle
        1, 2, 3, // 2nd triangle
    ];

    VAO::create();

    let vbo = BufferObject::create(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.store_f32_data(&vertices);

    let ibo = BufferObject::create(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
    ibo.store_i32_data(&indices);

    VertexAttribute::create_default(0, 3);
    VertexAttribute::create_default(1, 3);

    window.draw(|| unsafe {
        gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
    });
}

pub fn draw_triangle() {
    let mut window = Window::create(1080, 720, "Triangle");

    let vertices = [
        -0.5, -0.5, 0.0, // vertex 1
        0.5, -0.5, 0.0, // vertex 2
        0.0, 0.5, 0.0, // vertex 3
    ];

    VAO::create();

    let vbo = BufferObject::create(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.store_f32_data(&vertices);

    VertexAttribute::create_default(0, 3);

    window.draw(|| unsafe {
        gl::DrawArrays(gl::TRIANGLES, 0, 3);
    })
}
