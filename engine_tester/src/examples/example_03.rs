#![allow(dead_code)]
#![allow(unused_variables)]

use axis::{Mesh, Window};

pub fn draw_rect_and_triangle() {
    let mut window = Window::create(1080, 720, "Window");

    let rect = Mesh::new(
        &[
            -0.1, 0.5, 0.0, // top right
            -0.1, -0.5, 0.0, // bottom right
            -0.9, -0.5, 0.0, // bottom left
            -0.9, 0.5, 0.0, // top left
        ],
        Some(&[
            0, 1, 2, // 1st triangle
            0, 2, 3, // 2nd triangle
        ]),
    );
    rect.set_color((5.0, 0.3, 0.3, 1.0));

    let triangle = Mesh::new(
        &[
            0.5, 0.5, 0.0, // top
            0.9, -0.5, 0.0, // right
            0.1, -0.5, 0.0, // left
        ],
        None,
    );
    triangle.set_color((0.3, 0.3, 5.0, 1.0));

    window.draw(|| {
        rect.draw();
        triangle.draw();
    });
}

pub fn draw_rect() {
    let mut window = Window::create(1080, 720, "Window");

    let rect = Mesh::new(
        &[
            -0.1, 0.5, 0.0, // top right
            -0.1, -0.5, 0.0, // bottom right
            -0.9, -0.5, 0.0, // bottom left
            -0.9, 0.5, 0.0, // top left
        ],
        Some(&[
            0, 1, 2, // 1st triangle
            0, 2, 3, // 2nd triangle
        ]),
    );

    window.draw(|| {
        rect.draw();
    });
}

pub fn draw_triangle() {
    let mut window = Window::create(1080, 720, "Window");

    let triangle = Mesh::new(
        &[
            0.5, 0.5, 0.0, // top
            0.9, -0.5, 0.0, // right
            0.1, -0.5, 0.0, // left
        ],
        None,
    );

    window.draw(|| {
        triangle.draw();
    });
}
