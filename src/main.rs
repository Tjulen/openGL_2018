mod entity;
mod gl_buffers;
mod gl_vertex;
mod shader;
mod window;
extern crate gl;
use crate::gl_buffers::vert_buffer::Buffer;
use crate::entity::Entity;
use gl::types::*;
use nalgebra_glm::Vec4;
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
        profile: Some(glutin::GlProfile::Core),
        debug: false,
        robustness: glutin::Robustness::NoError,
        vsync: true,
    };

    let (gl_window, mut events_loop) = window::GameWindow::new(window_attribs, gl_attribs);

    //loop variables initialization
    let color_shaders = vec![
        shader::Shader::new(gl::VERTEX_SHADER, Path::new("shaders/vertex.glsl")),
        shader::Shader::new(gl::FRAGMENT_SHADER, Path::new("shaders/fragment.glsl")),
    ];
    let flat_shaders = vec![
        shader::Shader::new(gl::VERTEX_SHADER, Path::new("shaders/vertex_flat.glsl")),
        shader::Shader::new(gl::FRAGMENT_SHADER, Path::new("shaders/fragment_flat.glsl")),
    ];
    let color_program = shader::Program::new(&color_shaders);
    let flat_program = shader::Program::new(&flat_shaders);
    let background_color: [GLfloat; 4] = [0.2, 0.1, 0.3, 1.0];

    let col1 = [
        Vec4::new(0.0, 0.9, 0.6, 1.0),
        Vec4::new(0.5, 0.8, 0.2, 1.0),
        Vec4::new(0.1, 0.6, 0.2, 1.0),
    ];
    let pos1 = [
        Vec4::new(0.5, -0.5, 0.5, 1.0),
        Vec4::new(-0.5, -0.5, 0.5, 1.0),
        Vec4::new(0.0, 0.5, 0.5, 1.0),
    ];
    let mut buffer1 = Buffer::new(0, 0, gl::FLOAT, 4);
    let mut buffer2 = Buffer::new(1, 1, gl::FLOAT, 4);
    buffer1.array_data(&pos1, gl::STATIC_DRAW);
    buffer2.array_data(&col1, gl::STATIC_DRAW);
    let data = vec![buffer1, buffer2];
    let triangle = Entity::new(color_program, data);


    //rendering loop
    let mut running = true;
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
            gl::ClearBufferfv(gl::COLOR, 0, background_color.as_ptr() as *const GLfloat);
            triangle.draw();
        }

        match gl_window.swap_buffers() {
            Ok(_) => (),
            Err(error) => panic!("ERROR: {}", error),
        }
    }
}
