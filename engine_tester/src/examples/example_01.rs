#![allow(dead_code)]

use std::{mem, ptr};

use axis::{BufferObject, VertexAttribute, Window, VAO};

pub fn draw_rect_and_triangle() {
    let mut window = Window::new(1080, 720, "Window");
    window.background_color = (0.4, 0.5, 0.4, 1.0);
    window.init_gl();

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

    let rect_vao = VAO::new();
    rect_vao.bind();

    let rect_vbo = BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    rect_vbo.bind();
    rect_vbo.store_f32_data(&rect_vertices);

    let rect_ibo = BufferObject::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
    rect_ibo.bind();
    rect_ibo.store_i32_data(&rect_indices);

    let rect_position_attribute = VertexAttribute::new(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        3 * mem::size_of::<f32>() as i32,
        ptr::null(),
    );

    let rect_index_attribute = VertexAttribute::new(
        1,
        3,
        gl::FLOAT,
        gl::FALSE,
        3 * mem::size_of::<f32>() as i32,
        ptr::null(),
    );

    rect_position_attribute.enable();
    rect_index_attribute.enable();

    let triangle_vertices = [
        0.5, 0.5, 0.0, // top
        0.9, -0.5, 0.0, // right
        0.1, -0.5, 0.0, // left
    ];

    let triangle_vao = VAO::new();
    triangle_vao.bind();

    let triangle_vbo = BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    triangle_vbo.bind();
    triangle_vbo.store_f32_data(&triangle_vertices);

    let triangle_position_attribute = VertexAttribute::new(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        3 * mem::size_of::<f32>() as i32,
        ptr::null(),
    );
    triangle_position_attribute.enable();

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.3, 0.5, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            rect_vao.bind();
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
            rect_vao.unbind();

            triangle_vao.bind();
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            triangle_vao.unbind();
        }
        window.update();
    }
}

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
        3 * mem::size_of::<f32>() as i32,
        ptr::null(),
    );

    let index_attribute = VertexAttribute::new(
        1,
        3,
        gl::FLOAT,
        gl::FALSE,
        3 * mem::size_of::<f32>() as i32,
        ptr::null(),
    );

    position_attribute.enable();
    index_attribute.enable();

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.3, 0.5, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
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
        3 * mem::size_of::<f32>() as i32,
        ptr::null(),
    );
    position_attribute.enable();

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.3, 0.5, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
        window.update();
    }
}
