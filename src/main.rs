mod entity;
mod errors;
mod gl_buffers;
mod importer;
mod shader;
mod window;
extern crate gl;
extern crate nalgebra_glm as glm;
extern crate tobj;
extern crate image;
#[macro_use]
extern crate quick_error;
use gl_buffers::Image;
use game_time::step;
use game_time::FloatDuration;
use game_time::GameClock;
use gl::types::*;
use gl_buffers::{UniformBuffer, UniformType};
use std::path::Path;

fn main() {
    //initialization process
    let window_attribs = glutin::WindowAttributes {
        dimensions: Some(glutin::dpi::LogicalSize::new(1200.0, 600.0)),
        min_dimensions: None,
        max_dimensions: None,
        resizable: true,
        fullscreen: None,
        title: String::from("Bluto"),
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

    //shaders and programs initialization
    let tex_shader = vec![
        shader::Shader::new(gl::VERTEX_SHADER, Path::new("shaders/vert_texture.glsl")),
        shader::Shader::new(gl::FRAGMENT_SHADER, Path::new("shaders/frag_texture.glsl")),
    ];
    let tex_program = shader::Program::new(&tex_shader);

    let background_color: [GLfloat; 4] = [0.2, 0.1, 0.3, 1.0];

    //cubes entities instantiation
    let cube = importer::import_entity(Path::new("models/character_blue.obj"), &tex_program);
    let uniform_block = [
        UniformBuffer::new(0, UniformType::Image(
            Image::new("models/texture.png").unwrap()
        ))
    ];

    //time setup
    let mut clock = GameClock::new();
    let mut sim_time = clock.last_frame_time().clone();
    let step = step::ConstantStep::new(FloatDuration::milliseconds(60.0));
    let mut delta_time = 0.0;
    let mut current_time: f64 = 0.0;

    //rendering loop
    let mut running = true;
    while running {
        sim_time = clock.tick(&step);
        delta_time = sim_time.elapsed_wall_time().as_seconds();
        current_time = sim_time.total_game_time().as_seconds();

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
            gl::ClearBufferfi(gl::DEPTH_STENCIL, 0, 1.0, 0);
            cube.draw(&uniform_block);
        }

        match gl_window.swap_buffers() {
            Ok(_) => (),
            Err(error) => panic!("ERROR: {}", error),
        }
    }
}
