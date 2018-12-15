mod shader;

use gl::types::*;
use glutin::GlContext;
use std::path::Path;

struct Window<'a> {
    events_loop: &'a glutin::EventsLoop,
    window: &'a glutin::GlWindow,
}

impl<'a> Window<'a> {
    fn new(
        window_builder: &'a glutin::WindowAttributes,
        context_builder: &'a glutin::GlAttributes<&glutin::Context>,
        events_loop: &'a glutin::EventsLoop,
    ) -> Window<'a> {
        let events_loop = glutin::EventsLoop::new();
        let mut window_builder = glutin::WindowBuilder::new();
        window_builder.window = window_attribs;

        let mut context_builder = glutin::ContextBuilder::new();
        context_builder.gl_attr = gl_attribs;

        let window = glutin::GlWindow::new(window_builder, context_builder, &events_loop).unwrap();
        Window {
            events_loop: &events_loop,
            window: &window,
        }
    }
    fn new_events_loop() -> glutin::EventsLoop {
        glutin::EventsLoop::new()
    }
    fn new_context_builder(gl_attribs: glutin::GlAttributes<&glutin::Context>) -> glutin::ContextBuilder {
        let mut context_builder = glutin::ContextBuilder::new();
        context_builder.gl_attr = gl_attribs;
        context_builder
    }
    fn new_window_builder(window_attribs: glutin::WindowAttributes) -> glutin::WindowBuilder {
        let mut window_builder = glutin::WindowBuilder::new();
    	window_builder.window = window_attribs;
        window_builder
    }
    fn make_current(&mut self) {
        unsafe {
            self.window.make_current().unwrap();
        }
    }
}

// trait WindowAttribs {
//     fn bla() -> glutin::WindowAttributes;
// }

// impl WindowAttribs for glutin::WindowAttributes {
//     fn bla() -> glutin::WindowAttributes {

//     }
// }

fn main() {
    //initialization process
    let window_attribs = glutin::WindowAttributes {
        dimensions: Some(glutin::dpi::LogicalSize::new(1200.0, 600.0)),
        min_dimensions: None,
        max_dimensions: None,
        resizable: false,
        fullscreen: None,
        title: String::from("Bolty"),
        maximized: false,
        visible: true,
        transparent: false,
        decorations: true,
        always_on_top: false,
        window_icon: None,
        multitouch: false,
    };
    let gl_attribs = glutin::GlAttributes {
        sharing: None,
        version: glutin::GlRequest::Specific(glutin::Api::OpenGl, (4, 6)),
        profile: Some(glutin::GlProfile::Core),
        debug: true,
        robustness: glutin::Robustness::RobustNoResetNotification,
        vsync: true,
    };
    let mut gl_window = Window::new(window_attribs, gl_attribs);

    gl::load_with(|symbol| gl_window.window.get_proc_address(symbol) as *const _);

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
    while running {
        gl_window.events_loop.poll_events(|event| {
            if let glutin::Event::WindowEvent { event, .. } = event {
                match event {
                    glutin::WindowEvent::CloseRequested => running = false,
                    glutin::WindowEvent::Resized(logical_size) => gl_window
                        .window
                        .resize(logical_size.to_physical(gl_window.window.get_hidpi_factor())),
                    _ => (),
                }
            }
        });

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
