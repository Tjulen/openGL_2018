use crate::gl_buffers::vertex_array::VertexArray;
use crate::gl_buffers::attrib_buffer::AttribBuffer;
use crate::shader::Program;
use gl::types::*;

pub struct Mesh {
    vao: VertexArray,
    vbos: Vec<AttribBuffer>,
}

pub struct Entity {
    mesh: Mesh,
    program: Program,
}

impl Entity {
    pub fn new(program: Program, mut buffers: Vec<AttribBuffer>) -> Entity {
        let mut vao = VertexArray::new();
        for mut buffer in &mut buffers {
            Entity::assign_shader_binding(&program, &mut buffer);
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
            mesh: Mesh { vao, vbos: buffers },
            program,
        }
    }
    pub fn draw(&self) {
        self.mesh.bind();
        self.program.attach();
        unsafe {
            //TODO use a less error prone way of getting the amount of vertices to draw
            gl::DrawArrays(gl::TRIANGLES, 0 as GLint, self.mesh.vbos[0].vert_len);
        }
        self.program.detach();
        self.mesh.unbind();
    }
    //INFO gets the layout number of the buffer name and assigns it to the shader_binding field of buffer
    fn assign_shader_binding(program: &Program, buffer: &mut AttribBuffer) {
        let mut location = -1;
        unsafe {
            //ERR Cloned value buffer.name, may cause perf. downgrade
            //TODO better error handling
            let c_string_name = match std::ffi::CString::new(buffer.name.clone()) {
                Ok(c_string) => c_string,
                Err(err) => {
                    //INFO if throws this error it may be because your attrib is not used and is erased from shader
                    panic!("Could not create c_string from {}, {}", buffer.name, err);
                }
            };
            location =
                gl::GetAttribLocation(program.id, c_string_name.as_ptr() as *const i8);
        }
        if location == -1 {
            panic!("{} is not in shader", buffer.name);
        }
        buffer.shader_binding = location as i8;
    }
    //ERR function not stable or in working condition (may be removed)
    pub fn get_active_attribs(&self) {}
}

impl Mesh {
    pub fn bind(&self) {
        self.vao.bind();
    }
    pub fn unbind(&self) {
        self.vao.unbind();
    }
}
