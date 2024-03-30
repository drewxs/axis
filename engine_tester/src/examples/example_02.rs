#![allow(dead_code)]

use std::ptr;

use axis::{BufferObject, VertexAttribute, Window, VAO};

pub fn draw_rect_and_triangle() {
    let mut window = Window::create(1080, 720, "Window");

    let rect_vertices = [
        -0.1, 0.5, 0.0, // top right
        -0.1, -0.5, 0.0, // bottom right
        -0.9, -0.5, 0.0, // bottom left
        -0.9, 0.5, 0.0, // top left
    ];
    let rect_indices = [
        0, 1, 2, // 1st triangle
        0, 2, 3, // 2nd triangle
    ];

    let rect_vao = VAO::create();

    let rect_vbo = BufferObject::create(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    rect_vbo.store_f32_data(&rect_vertices);

    let rect_ibo = BufferObject::create(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
    rect_ibo.store_i32_data(&rect_indices);

    VertexAttribute::create_default(0, 3);
    VertexAttribute::create_default(1, 3);

    let triangle_vertices = [
        0.5, 0.5, 0.0, // top
        0.9, -0.5, 0.0, // right
        0.1, -0.5, 0.0, // left
    ];

    let triangle_vao = VAO::create();

    let triangle_vbo = BufferObject::create(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    triangle_vbo.store_f32_data(&triangle_vertices);

    VertexAttribute::create_default(0, 3);

    window.draw(|| unsafe {
        rect_vao.bind();
        gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());

        triangle_vao.bind();
        gl::DrawArrays(gl::TRIANGLES, 0, 3);
    });
}

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
