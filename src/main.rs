mod shader;
mod window;

use gl::types::*;
use std::path::Path;

fn main() {
    //initialization process
    let window_attribs = glutin::WindowAttributes {
        dimensions: Some(glutin::dpi::LogicalSize::new(1200.0, 600.0)),
        min_dimensions: None,
        max_dimensions: None,
        resizable: true,
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
        version: glutin::GlRequest::Latest,
        profile: None,
        debug: false,
        robustness: glutin::Robustness::NoError,
        vsync: true,
    };
    let (gl_window, mut events_loop) = window::GameWindow::new(window_attribs, gl_attribs);

    //VERY IMPORTANT SEQUENCE - if load_gl_ptr before make_current it throws an error - cannot load fn ptr
    gl_window.make_current();
    gl_window.load_gl_ptr();

    //loop variables initialization
    let shaders = vec![
        shader::Shader::new(gl::VERTEX_SHADER, Path::new("shaders/vertex.glsl")),
        shader::Shader::new(gl::FRAGMENT_SHADER, Path::new("shaders/fragment.glsl")),
    ];
    let rendering_program = shader::Program::new(&shaders);
    const background_color: [GLfloat; 4] = [137.0/255.0, 176.0/255.0, 174.0/255.0, 1.0];

    //rendering loop
    let mut running = true;
    rendering_program.activate();
    while running {
        events_loop.poll_events(|event| {
            if let glutin::Event::WindowEvent { event, .. } = event {
                match event {
                    glutin::WindowEvent::CloseRequested => running = false,
                    glutin::WindowEvent::Resized(logical_size) => gl_window.resize(logical_size),
                    _ => (),
                }
            }
        });

        unsafe {
            gl::ClearBufferfv(gl::COLOR, 0, &background_color as *const GLfloat);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        match gl_window.swap_buffers() {
            Ok(_) => (),
            Err(error) => panic!("ERROR: {}", error),
        }
    }
}
