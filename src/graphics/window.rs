use glfw::{
    fail_on_errors, Action, Context, Glfw, GlfwReceiver, Key, PWindow, WindowEvent, WindowMode,
};

pub struct Window {
    glfw: Glfw,
    window_handle: PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,
    pub background_color: (f32, f32, f32, f32),
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Window {
        let mut glfw = glfw::init(fail_on_errors).unwrap();

        let (mut window, events) = glfw
            .create_window(width, height, title, WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.set_framebuffer_size_polling(true);
        window.set_key_polling(true);

        Window {
            glfw,
            window_handle: window,
            events,
            background_color: (0.0, 0.0, 0.0, 0.0),
        }
    }

    pub fn create(width: u32, height: u32, title: &str) -> Window {
        let mut window = Window::new(width, height, title);
        window.init_gl();
        window
    }

    pub fn init_gl(&mut self) {
        self.window_handle.make_current();
        gl::load_with(|s| self.window_handle.get_proc_address(s) as *const _);
    }

    pub fn should_close(&self) -> bool {
        self.window_handle.should_close()
    }

    pub fn draw<F: Fn()>(&mut self, render_fn: F) {
        while !self.should_close() {
            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT);
                let (r, g, b, a) = self.background_color;
                gl::ClearColor(r, g, b, a);
                render_fn();
            }
            self.update();
        }
    }

    pub fn update(&mut self) {
        self.process_events();
        self.glfw.poll_events();
        self.window_handle.swap_buffers();
    }

    fn process_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window_handle.set_should_close(true);
                }
                _ => {}
            }
        }
    }
}
