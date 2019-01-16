mod entity;
mod errors;
mod gl_buffers;
mod gl_vertex;
mod importer;
mod shader;
mod window;
extern crate gl;
extern crate nalgebra_glm as na;
extern crate tobj;
#[macro_use]
extern crate quick_error;
use gl::types::*;
use std::path::Path;
use na::vec3;
use game_time::GameClock;
use game_time::step;
use game_time::FloatDuration;


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

    //variables initialization
    let flat_shaders = vec![
        shader::Shader::new(gl::VERTEX_SHADER, Path::new("shaders/vertex_flat.glsl")),
        shader::Shader::new(gl::FRAGMENT_SHADER, Path::new("shaders/fragment_flat.glsl")),
    ];
    let flat_program = shader::Program::new(&flat_shaders);
    let background_color: [GLfloat; 4] = [0.2, 0.1, 0.3, 1.0];

    let cube = importer::import_entity(std::path::Path::new("models/cube.obj"), &flat_program);

    //time setup
    let mut clock = GameClock::new();
    let mut sim_time = clock.last_frame_time().clone();
    let step = step::ConstantStep::new(FloatDuration::milliseconds(50.0));
    let mut delta_time = 0.0;

    //rendering loop
    let mut running = true;
    while running {
        sim_time = clock.tick(&step);
        delta_time = sim_time.elapsed_wall_time().as_seconds();
        println!("{}", delta_time);
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
            cube.draw();
        }

        match gl_window.swap_buffers() {
            Ok(_) => (),
            Err(error) => panic!("ERROR: {}", error),
        }
    }
}
