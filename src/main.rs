use gl::types::*;
use glutin::GlContext;

static VERTEX_SHADER: &str = r#"
#version 450 core

void main() {
    const vec4 vertices[3] = vec4[3] (vec4( 0.25, -0.25, 0.5, 1.0),
                                      vec4(-0.25, -0.25, 0.5, 1.0),
                                      vec4( 0.25,  0.25, 0.5, 1.0));
    gl_Position = vertices[gl_VertexID]; //+ offset
}
"#;

static TESSELLATION_EVALUATION_SHADER: &str = r#"
#version 450 core

layout (triangles, equal_spacing, cw) in;

void main() {
    gl_Position = (gl_TessCoord.x * gl_in[0].gl_Position +
                   gl_TessCoord.y * gl_in[1].gl_Position +
                   gl_TessCoord.z * gl_in[2].gl_Position);
}
"#;

static TESSELLATION_CONTROL_SHADER: &str = r#"
#version 450 core

layout (vertices = 3) out;

void main() {
    gl_TessLevelInner[0] = 5.0;
    gl_TessLevelOuter[0] = 5.0;
    gl_TessLevelOuter[1] = 5.0;
    gl_TessLevelOuter[2] = 5.0;

    gl_out[gl_InvocationID].gl_Position = gl_in[gl_InvocationID].gl_Position;
}
"#;

static FRAGMENT_SHADER: &str = r#"
#version 450 core

//in vec4 vs_color;

out vec4 color;

void main(){
    color = vec4(1.0, 1.0, 1.0, 1.0);
}
"#;

fn main() {
    //initialization process
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Hello, world")
        .with_dimensions(glutin::dpi::LogicalSize::new(1200.0, 800.0));
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();
    unsafe {
        gl_window.make_current().unwrap();
    }
    gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);

    //loop variables initialization
    let rendering_program = compile_shaders(VERTEX_SHADER, FRAGMENT_SHADER, Some(TESSELLATION_CONTROL_SHADER), Some(TESSELLATION_EVALUATION_SHADER));
    let background_color: [GLfloat; 4] = [0.2, 0.0, 0.2, 1.0];

    //rendering loop
    let mut running = true;
    while running {
        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => running = false,
                glutin::WindowEvent::Resized(logical_size) => {
                    gl_window.resize(logical_size.to_physical(gl_window.get_hidpi_factor()))
                }
                _ => (),
            },
            _ => (),
        });

        let attrib: [GLfloat; 4] = [0.5, 0.6, 0.0, 0.0];

        unsafe {
            gl::ClearBufferfv(gl::COLOR, 0, &background_color as *const GLfloat);
            gl::UseProgram(rendering_program);
            //gl::VertexAttrib4fv(1, &attrib as *const GLfloat);
            gl::PatchParameteri(gl::PATCH_VERTICES, 3);
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            gl::DrawArrays(gl::PATCHES, 0, 3);
        }

        match gl_window.swap_buffers() {
            Ok(_) => (),
            Err(error) => panic!("ERROR: {}", error),
        }
    }

    unsafe {
        gl::DeleteProgram(rendering_program);
    }
}

fn compile_shaders(
    vertex_source: &str,
    fragment_source: &str,
    tessellation_control_source: Option<&str>,
    tessellation_evaluation_source: Option<&str>,
) -> GLuint {
    let vertex_shader: GLuint;
    let fragment_shader: GLuint;
    let tessellation_control_shader: GLuint;
    let tessellation_evaluation_shader: GLuint;
    let program: GLuint;

    //compiling the vertex shader
    unsafe {
        vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        let c_str_vertex = {
            match std::ffi::CString::new(vertex_source.as_bytes()) {
                Ok(c_string) => c_string,
                Err(error) => panic!("ERROR: {}", error),
            }
        };
        gl::ShaderSource(vertex_shader, 1, &c_str_vertex.as_ptr(), std::ptr::null());
        gl::CompileShader(vertex_shader);

        //error check -- essential!!
        let mut shader_compile_success = gl::FALSE as i32;
        let mut info_log = Vec::with_capacity(512);
        info_log.set_len(512 - 1);

        gl::GetShaderiv(
            vertex_shader,
            gl::COMPILE_STATUS,
            &mut shader_compile_success,
        );
        if shader_compile_success != gl::TRUE as i32 {
            gl::GetShaderInfoLog(
                vertex_shader,
                512,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut gl::types::GLchar,
            );

            println!(
                "ERROR: vertex shader compiled unsuccessfully, log: {}",
                std::str::from_utf8(&info_log).unwrap()
            );
        };
    }

    //compiling the tessellation control shader
    match tessellation_control_source {
        Some(tessellation_control_source) => unsafe {
            tessellation_control_shader = gl::CreateShader(gl::TESS_CONTROL_SHADER);
            let c_str_tess_control = {
                match std::ffi::CString::new(tessellation_control_source.as_bytes()) {
                    Ok(c_string) => c_string,
                    Err(error) => panic!("ERROR: {}", error),
                }
            };
            gl::ShaderSource(
                tessellation_control_shader,
                1,
                &c_str_tess_control.as_ptr(),
                std::ptr::null(),
            );
            gl::CompileShader(tessellation_control_shader);

            //error check -- essential!!
            let mut shader_compile_success = gl::FALSE as i32;
            let mut info_log = Vec::with_capacity(512);
            info_log.set_len(512 - 1);

            gl::GetShaderiv(
                tessellation_control_shader,
                gl::COMPILE_STATUS,
                &mut shader_compile_success,
            );
            if shader_compile_success != gl::TRUE as i32 {
                gl::GetShaderInfoLog(
                    tessellation_control_shader,
                    512,
                    std::ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut gl::types::GLchar,
                );

                println!(
                    "ERROR: tessellation control shader compiled unsuccessfully, log: {}",
                    std::str::from_utf8(&info_log).unwrap()
                );
            }
        },

        None => tessellation_control_shader = 0,
    }

    //compilint the tessellation evaluation shader
    match tessellation_evaluation_source {
        Some(tessellation_evaluation_source) => unsafe {
            tessellation_evaluation_shader = gl::CreateShader(gl::TESS_EVALUATION_SHADER);
            let c_str_tess_evaluation = {
                match std::ffi::CString::new(tessellation_evaluation_source.as_bytes()) {
                    Ok(c_string) => c_string,
                    Err(error) => panic!("ERROR: {}", error),
                }
            };
            gl::ShaderSource(
                tessellation_evaluation_shader,
                1,
                &c_str_tess_evaluation.as_ptr(),
                std::ptr::null(),
            );
            gl::CompileShader(tessellation_evaluation_shader);

            //error check -- essential!!
            let mut shader_compile_success = gl::FALSE as i32;
            let mut info_log = Vec::with_capacity(512);
            info_log.set_len(512 - 1);

            gl::GetShaderiv(
                tessellation_evaluation_shader,
                gl::COMPILE_STATUS,
                &mut shader_compile_success,
            );
            if shader_compile_success != gl::TRUE as i32 {
                gl::GetShaderInfoLog(
                    tessellation_evaluation_shader,
                    512,
                    std::ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut gl::types::GLchar,
                );

                println!(
                    "ERROR: tessellation evaluation shader compiled unsuccessfully, log: {}",
                    std::str::from_utf8(&info_log).unwrap()
                );
            }
        },
        None => tessellation_evaluation_shader = 0,
    }

    //compiling the fragment shader
    unsafe {
        fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        let c_str_fragment = {
            match std::ffi::CString::new(fragment_source.as_bytes()) {
                Ok(c_string) => c_string,
                Err(error) => panic!("ERROR: {}", error),
            }
        };
        gl::ShaderSource(
            fragment_shader,
            1,
            &c_str_fragment.as_ptr(),
            std::ptr::null(),
        );
        gl::CompileShader(fragment_shader);

        //error check -- essential !!
        let mut shader_compile_success = gl::FALSE as i32;
        let mut info_log = Vec::with_capacity(512);
        info_log.set_len(512 - 1);

        gl::GetShaderiv(
            fragment_shader,
            gl::COMPILE_STATUS,
            &mut shader_compile_success,
        );
        if shader_compile_success != gl::TRUE as i32 {
            gl::GetShaderInfoLog(
                fragment_shader,
                512,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut gl::types::GLchar,
            );

            println!(
                "ERROR: fragment shader compiled unsuccessfully, log: {}",
                std::str::from_utf8(&info_log).unwrap()
            );
        };
    }

    //create the program
    unsafe {
        program = gl::CreateProgram();
        gl::AttachShader(program, fragment_shader);
        gl::AttachShader(program, vertex_shader);
        if tessellation_control_shader != 0 {
            gl::AttachShader(program, tessellation_control_shader);
        }
        if tessellation_evaluation_shader != 0 {
            gl::AttachShader(program, tessellation_evaluation_shader);
        }
        gl::LinkProgram(program);

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
        if tessellation_control_shader != 0 {
            gl::DeleteShader(tessellation_control_shader);
        }
        if tessellation_evaluation_shader != 0 {
            gl::DeleteShader(tessellation_evaluation_shader);
        }

        //linking error check
        let mut program_compile_success = gl::FALSE as i32;
        let mut info_log = Vec::with_capacity(512);
        info_log.set_len(512 - 1);

        gl::GetProgramiv(program, gl::LINK_STATUS, &mut program_compile_success);
        if program_compile_success != gl::TRUE as i32 {
            gl::GetProgramInfoLog(
                program,
                512,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut gl::types::GLchar,
            );

            println!(
                "ERROR: program compilation failed, log: {}",
                std::str::from_utf8(&info_log).unwrap()
            );
        };
    }

    program
}
