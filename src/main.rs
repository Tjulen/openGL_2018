mod gl_buffers;
mod shader;
mod window;
mod gl_vertex;

use crate::gl_buffers::dynamic_buffer::DynamicBuffer;
use crate::gl_buffers::vertex_array::Vao;
use crate::gl_vertex::Vertex;
use crate::gl_vertex::VertexAttribute;
use gl::types::*;
use nalgebra_glm::Vec4;
use std::path::Path;

struct color_vert {
    pos: Vec4,
    col: Vec4,
}

impl Vertex for color_vert {
    #[inline]
    fn vert_size() -> usize {
        std::mem::size_of::<Self>()
    }
    fn attribs() -> Vec<Box<VertexAttribute>> {
        vec![
            Box::new(Vec4::new(0.0, 0.0, 0.0, 0.0)),
            Box::new(Vec4::new(0.0, 0.0, 0.0, 0.0)),
        ]
    }
}

struct flat_vert {
    pos: Vec4,
}

impl Vertex for flat_vert {
    fn attribs() -> Vec<Box<VertexAttribute>> {
        vec![Box::new(Vec4::new(0.0, 0.0, 0.0, 0.0))]
    }
    #[inline]
    fn vert_size() -> usize {
        std::mem::size_of::<Self>()
    }
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
    let color_vertices = [
        color_vert {
            pos: Vec4::new(0.5, -0.5, 0.5, 1.0),
            col: Vec4::new(0.2, 0.1, 1.0, 1.0),
        },
        color_vert {
            pos: Vec4::new(-0.5, -0.5, 0.5, 1.0),
            col: Vec4::new(1.0, 0.4, 0.3, 1.0),
        },
        color_vert {
            pos: Vec4::new(0.0, 0.5, 0.5, 1.0),
            col: Vec4::new(0.2, 0.9, 0.2, 1.0),
        },
    ];
    let flat_vertices = [
        flat_vert {
            pos: Vec4::new(0.8, -0.8, 0.5, 1.0),
        },
        flat_vert {
            pos: Vec4::new(-0.8, -0.8, 0.5, 1.0),
        },
        flat_vert {
            pos: Vec4::new(-0.8, 0.8, 0.5, 1.0),
        },
    ];
    let mut color_buffer = DynamicBuffer::new();
    color_buffer.array_data(&color_vertices);
    let mut flat_buffer = DynamicBuffer::new();
    flat_buffer.array_data(&flat_vertices);
    let color_vertex_array = Vao::new(color_vert::vert_size() as i32);
    let flat_vertex_array = Vao::new(flat_vert::vert_size() as i32);
    color_vertex_array.bind_buffer(color_buffer.id());
    color_vertex_array.attrib_setup(color_vert::attribs());
    flat_vertex_array.bind_buffer(flat_buffer.id());
    flat_vertex_array.attrib_setup(flat_vert::attribs());

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
            color_program.enable();
            color_vertex_array.bind();
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            color_vertex_array.unbind();
            flat_vertex_array.bind();
            flat_program.enable();
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            flat_vertex_array.unbind();
        }

        match gl_window.swap_buffers() {
            Ok(_) => (),
            Err(error) => panic!("ERROR: {}", error),
        }
    }
}
