use crate::gl::types::*;
use std::ffi::CString;
use std::io::Read;
use std::path::Path;
use crate::errors::EngineError;

type ShaderType = GLenum;

pub struct Program {
    pub id: u32,
}
//No impl Drop, because it gets discarded and deleted in program creation (Program::new())
pub struct Shader {
    pub id: u32,
}
impl Shader {
    pub fn new(s_type: ShaderType, path: &Path) -> Shader {
        let string = match path_to_string(path) {
            Ok(string) => string,
            Err(_error) => panic!("ERROR: cannot find {}", path.to_str().unwrap()),
        };
        let c_string = match CString::new(string.as_bytes()) {
            Ok(c_string) => c_string,
            Err(error) => panic!("ERROR: {}", error),
        };
        let shader_id = Shader::compile_shader(s_type, c_string);

        Shader { id: shader_id }
    }
    fn compile_shader(s_type: ShaderType, source: CString) -> GLuint {
        let shader_id = unsafe { gl::CreateShader(s_type) };
        unsafe {
            gl::ShaderSource(shader_id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(shader_id);
        }
        Shader::shader_error_check(shader_id);
        shader_id
    }
    fn shader_error_check(shader_id: GLuint) {
        let mut compile_success = i32::from(gl::FALSE);
        let mut info_log = Vec::with_capacity(512);
        //in c, the string has a null terminator at the end, which takes 1 byte
        unsafe {
            info_log.set_len(512 - 1);
            gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut compile_success);
        }

        if compile_success == i32::from(gl::FALSE) {
            unsafe {
                gl::GetShaderInfoLog(
                    shader_id,
                    512,
                    std::ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
            }
            panic!("ERROR: {}", std::str::from_utf8(&info_log).unwrap());
        }
    }
}

impl Program {
    pub fn new(shaders: &[Shader]) -> Program {
        let program_id = unsafe { gl::CreateProgram() };
        for shader in shaders {
            unsafe {
                gl::AttachShader(program_id, shader.id);
            }
        }
        unsafe {
            gl::LinkProgram(program_id);
        }

        for shader in shaders {
            unsafe {
                gl::DetachShader(program_id, shader.id);
                gl::DeleteShader(shader.id);
            }
        }
        Program::program_linking_check(program_id);
        Program { id: program_id }
    }
    fn program_linking_check(program_id: GLuint) {
        let mut link_success = i32::from(gl::FALSE);
        let mut info_log = Vec::with_capacity(512);
        unsafe {
            info_log.set_len(512 - 1);
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut link_success);
        }

        if link_success == i32::from(gl::FALSE) {
            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    512,
                    std::ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
            }
            panic!("ERROR: {}", std::str::from_utf8(&info_log).unwrap());
        }
    }
    #[inline]
    pub fn attach(&self) {
        unsafe { gl::UseProgram(self.id) }
    }
    #[inline]
    pub fn detach(&self) {
        unsafe { gl::UseProgram(0) }
    }
    #[inline]
    pub fn get_attrib_location(&self, name: &CString) -> Result<i8, EngineError> {
        let mut _location = -1;
        unsafe {
            _location = gl::GetAttribLocation(self.id, name.as_ptr() as *const i8) as i8;
        }
        if _location == -1 {
            //ERR tidious decision of creating this error, but only created when something is wrong, so it is not so slow
            return Err(EngineError::GetAttrib(name.clone().into_string().unwrap()));
        }
        Ok(_location)
    }
    #[inline]
    pub fn get_uniform_location(&self, name: &CString) -> Result<i8, EngineError> {
        let mut _location = -1;
        unsafe {
            _location = gl::GetUniformLocation(self.id, name.as_ptr() as *const i8) as i8;
        }
        if _location == -1 {
            return Err(EngineError::GetAttrib(name.clone().into_string().unwrap()));
        }
        Ok(_location)
    }
}
impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

fn path_to_string(path: &std::path::Path) -> std::io::Result<String> {
    let mut file = std::fs::File::open(path)?;
    let mut string: String = String::new();
    file.read_to_string(&mut string)?;
    Ok(string)
}


