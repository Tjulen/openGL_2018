use crate::gl_buffers::attrib_buffer::AttribBuffer;
use crate::gl_buffers::vertex_array::VertexArray;
use crate::shader::Program;
use crate::errors::EngineError;
use gl::types::*;

pub struct Mesh {
    vao: VertexArray,
    vbos: Vec<AttribBuffer>,
    triangle_count: u64
}

pub struct Entity<'a> {
    mesh: Mesh,
    program: &'a Program,
}

impl<'a> Entity<'a> {
    pub fn new<'b>(program: &'b Program, mut buffers: Vec<AttribBuffer>, triangle_count: u64) -> Entity {
        let mut vao = VertexArray::new();
        for mut buffer in &mut buffers {
            match Entity::assign_shader_binding(&program, &mut buffer) {
                Ok(_) => (),
                Err(err) => {
                    println!("{}", err);
                    //if continue this is not here, somehow eventhough the shader_binding is -1 the buffer is still bound correctly; maybe because casting from i8 to u32 in vao::setup_buffer is not prohibited and -1 gets cast to unsigned
                    continue;
                },
            };
            vao.setup_attrib(&mut buffer);
            unsafe {
                gl::VertexArrayVertexBuffer(
                    vao.id,
                    buffer.vao_binding as GLuint,
                    buffer.id,
                    0 as GLintptr,
                    buffer.size,
                );
            }
        }
        Entity {
            mesh: Mesh { vao, vbos: buffers, triangle_count },
            program,
        }
    }
    pub fn draw(&self) {
        self.mesh.bind();
        self.program.attach();
        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0 as GLint, self.mesh.triangle_count as i32 * 3);
        }
        self.program.detach();
        self.mesh.unbind();
    }

    //INFO gets the layout number of the buffer name and assigns it to the shader_binding field of buffer
    fn assign_shader_binding(
        program: &Program,
        buffer: &mut AttribBuffer,
    ) -> Result<(), EngineError> {
        //ERR Cloned value buffer.name, may cause perf. downgrade
        let c_string_name = match std::ffi::CString::new(buffer.name.clone()) {
            Ok(c_string) => c_string,
            Err(err) => {
                let message = format!("{}, {}", buffer.name, err).to_string();
                return Err(EngineError::CStringCreation(message));
            }
        };
        //INFO if throws this error it may be because your attrib is not used and is erased from shader
        let location = program.get_attrib_location(&c_string_name)?;
        buffer.shader_binding = location as i8;
        Ok(())
    }
}

impl Mesh {
    pub fn bind(&self) {
        self.vao.bind();
    }
    pub fn unbind(&self) {
        self.vao.unbind();
    }
}
