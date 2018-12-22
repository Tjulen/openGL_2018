mod shader;
mod window;
#[macro_use]
mod macros;

use gl::types::*;
use nalgebra::Vector4;
use std::path::Path;

struct Vertex {
    pos: Vector4<f32>,
    col: Vector4<f32>,
}

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

    #[warn(unused_mut)]
    let (gl_window, mut events_loop) = window::GameWindow::new(window_attribs, gl_attribs);

    //loop variables initialization
    let shaders = vec![
        shader::Shader::new(gl::VERTEX_SHADER, Path::new("shaders/vertex.glsl")),
        shader::Shader::new(gl::FRAGMENT_SHADER, Path::new("shaders/fragment.glsl")),
    ];
    let rendering_program = shader::Program::new(&shaders);
    let background_color: [GLfloat; 4] = [137.0 / 255.0, 176.0 / 255.0, 174.0 / 255.0, 1.0];
    let vertices = [
        Vertex {
            pos: Vector4::new(0.5, -0.5, 0.5, 1.0),
            col: Vector4::new(0.2, 0.1, 1.0, 1.0),
        },
        Vertex {
            pos: Vector4::new(-0.5, -0.5, 0.5, 1.0),
            col: Vector4::new(1.0, 0.4, 0.3, 1.0),
        },
        Vertex {
            pos: Vector4::new(0.0, 0.5, 0.5, 1.0),
            col: Vector4::new(0.2, 0.9, 0.2, 1.0),
        },
    ];
    let mut buffer_id = 0;
    let mut vertex_array_id = 0;
    unsafe {
        gl::CreateBuffers(1, &mut buffer_id);
        gl::NamedBufferStorage(buffer_id, 3 * std::mem::size_of::<Vertex>() as isize, vertices.as_ptr() as *const std::ffi::c_void, gl::MAP_WRITE_BIT);

        gl::CreateVertexArrays(1, &mut vertex_array_id);
        gl::VertexArrayAttribFormat(vertex_array_id, 0, 4, gl::FLOAT, gl::FALSE, 0);
        gl::VertexArrayAttribFormat(
            vertex_array_id,
            1,
            4,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<Vector4<f32>>() as u32,
        );
        gl::VertexArrayVertexBuffer(vertex_array_id, 0, buffer_id, 0, std::mem::size_of::<Vertex>() as i32);
        gl::VertexArrayAttribBinding(vertex_array_id, 0, 0);
        gl::VertexArrayAttribBinding(vertex_array_id, 1, 0);
        gl::EnableVertexArrayAttrib(vertex_array_id, 0);
        gl::EnableVertexArrayAttrib(vertex_array_id, 1);
        gl::BindVertexArray(vertex_array_id);
    }

    //rendering loop
    let mut running = true;
    rendering_program.enable();
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
