use crate::gl_buffers::vert_buffer::Buffer;
use crate::gl_buffers::vertex_array::VertexArray;
use crate::shader::Program;
use gl::types::*;

pub struct Mesh {
    vao: VertexArray,
    vbos: Vec<Buffer>,
}

pub struct Entity {
    mesh: Mesh,
    program: Program,
}

impl Entity {
    pub fn new(program: Program, buffers: Vec<Buffer>) -> Entity {
        let vao = VertexArray::new();
        for buffer in &buffers {
            vao.setup_attrib(buffer);
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
            gl::DrawArrays(gl::TRIANGLES, 0 as GLint, self.mesh.vbos[0].vert_len);
        }
        self.program.detach();
        self.mesh.unbind();
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
