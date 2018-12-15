mod shader;

use gl::types::*;
use glutin::GlContext;
use std::path::Path;

struct Window {
    events_loop: glutin::EventsLoop,
    window: glutin::GlWindow,
}

impl Window {
    fn new(
        window_builder: glutin::WindowBuilder,
        context_builder: glutin::ContextBuilder,
        events_loop: glutin::EventsLoop,
    ) -> Window {
        let window = glutin::GlWindow::new(window_builder, context_builder, &events_loop).unwrap();
        Window {
            events_loop: events_loop,
            window: window,
        }
    }
    fn new_events_loop() -> glutin::EventsLoop {
        glutin::EventsLoop::new()
    }
    fn new_context_builder(
        gl_attribs: glutin::GlAttributes<&glutin::Context>,
    ) -> glutin::ContextBuilder {
        let mut context_builder = glutin::ContextBuilder::new();
        context_builder.gl_attr = gl_attribs;
        context_builder
    }
    fn new_window_builder(window_attribs: glutin::WindowAttributes) -> glutin::WindowBuilder {
        let mut window_builder = glutin::WindowBuilder::new();
        window_builder.window = window_attribs;
        window_builder
    }
    fn resize(&self, logical_size: glutin::dpi::LogicalSize) {
        unsafe {
            gl::Viewport(0, 0, logical_size.height as i32, logical_size.width as i32);
        }
    }
    fn load_gl_ptr(&self) {
        gl::load_with(|symbol| self.window.get_proc_address(symbol) as *const _);
    }
    fn make_current(&mut self) {
        unsafe {
            self.window.make_current().unwrap();
        }
    }
    fn fetch_events<F>(&mut self, callback: F)
    where
        F: FnMut(glutin::Event),
    {
        self.events_loop.poll_events(callback);
    }
}

fn main() {
    //initialization process
    let window_attribs = glutin::WindowAttributes::default();
    let gl_attribs = glutin::GlAttributes::default();
    let mut gl_window = Window::new(
        Window::new_window_builder(window_attribs),
        Window::new_context_builder(gl_attribs),
        Window::new_events_loop(),
    );

    //VERY IMPORTANT SEQUENCE - if load_gl_ptr before make_current it throws an error - cannot load fn ptr
    gl_window.make_current();
    gl_window.load_gl_ptr();

    //loop variables initialization
    let shaders = vec![
        shader::Shader::new(gl::VERTEX_SHADER, Path::new("shaders/vertex.glsl")),
        shader::Shader::new(gl::FRAGMENT_SHADER, Path::new("shaders/fragment.glsl")),
        shader::Shader::new(
            gl::TESS_EVALUATION_SHADER,
            Path::new("shaders/tess_eval.glsl"),
        ),
        shader::Shader::new(
            gl::TESS_CONTROL_SHADER,
            Path::new("shaders/tess_control.glsl"),
        ),
    ];
    let rendering_program = shader::Program::new(&shaders);
    let background_color: [GLfloat; 4] = [0.2, 0.0, 0.2, 1.0];

    //rendering loop
    let mut running = true;
    let mut resized = false;
    let mut resize_logical_size: glutin::dpi::LogicalSize =
        glutin::dpi::LogicalSize::new(12.0, 12.0);
    while running {
        gl_window.fetch_events(|event| {
            if let glutin::Event::WindowEvent { event, .. } = event {
                match event {
                    glutin::WindowEvent::CloseRequested => running = false,
                    glutin::WindowEvent::Resized(logical_size) => {
                        resize_logical_size = logical_size;
                    }
                    _ => (),
                }
            }
        });
        if resized {
            resized = false;
            gl_window.resize(resize_logical_size);
        }

        //let attrib: [GLfloat; 4] = [0.5, 0.6, 0.0, 0.0];

        unsafe {
            gl::ClearBufferfv(gl::COLOR, 0, &background_color as *const GLfloat);
            rendering_program.activate();
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            gl::DrawArrays(gl::PATCHES, 0, 3);
        }

        match gl_window.window.swap_buffers() {
            Ok(_) => (),
            Err(error) => panic!("ERROR: {}", error),
        }
    }
}
