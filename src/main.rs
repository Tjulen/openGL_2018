mod entity;
mod errors;
mod gl_buffers;
mod importer;
mod shader;
mod window;
extern crate gl;
extern crate nalgebra_glm as glm;
extern crate tobj;
#[macro_use]
extern crate quick_error;
use game_time::step;
use game_time::FloatDuration;
use game_time::GameClock;
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
    let step = step::ConstantStep::new(FloatDuration::milliseconds(60.0));
    let mut delta_time = 0.0;
    let mut current_time: f64 = 0.0;

    //spinning cube
    // let f: f64 = current_time * std::f64::consts::PI * 0.1;
    // let mut mv_matrix: glm::Mat4 = glm::Mat4::new_translation(&glm::Vec3::new(0.0, 0.0, -4.0));
    // mv_matrix.append_translation(&glm::Vec3::new(
    //     (2.1 * f).sin() as f32 * 0.5,
    //     (1.7 * f).sin() as f32 * 0.5,
    //     (1.3 * f).sin() as f32 * (1.5 * f).cos() as f32 * 2.0,
    // ));
    // mv_matrix = mv_matrix * glm::rotation(current_time as f32 * 45.0, &glm::Vec3::new(0.0, 1.0, 0.0)) * glm::rotation(current_time as f32 * 81.0, &glm::Vec3::new(1.0, 0.0, 0.0));
    // let proj_matrix = glm::perspective(2.0, 50.0, 0.1, 1000.0);

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
            cube.draw(current_time, delta_time);
        }

        match gl_window.swap_buffers() {
            Ok(_) => (),
            Err(error) => panic!("ERROR: {}", error),
        }
    }
}
