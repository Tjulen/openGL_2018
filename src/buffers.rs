use crate::vertex::Vertex;
use gl::types::*;

pub struct simple_vbo {
    id: GLuint,
    vert_size: usize,
}

impl simple_vbo {
    pub fn from_vec(data: Vec<impl Vertex>) -> simple_vbo {
        let mut vbo_id = 0;
        unsafe {
            gl::CreateBuffers(1, &mut vbo_id);
        }
        let vert_size = data[0].size_of() as usize;
        let simple_vbo = simple_vbo {
            id: vbo_id,
            vert_size,
        };
        unsafe {
            gl::NamedBufferStorage(
                vbo_id,
                (vert_size * data.len()) as isize,
                data.as_ptr() as *const std::ffi::c_void,
                gl::DYNAMIC_STORAGE_BIT,
            );
        }

        simple_vbo
    }
    pub fn id(&self) -> GLuint {
        self.id
    }
    pub fn vert_size(&self) -> usize {
        self.vert_size
    }
}

impl Drop for simple_vbo {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}

pub struct simple_vao {
    id: GLuint,
}

impl simple_vao {
    pub fn new() -> simple_vao {
        let mut vao_id = 0;
        unsafe {
            gl::CreateVertexArrays(1, &mut vao_id);
        }

        simple_vao { id: vao_id }
    }

    //specifies the binding index from which the vao will get its source when it is bound to be used
    pub fn vertex_array_vertex_buffer(&self, buffer: simple_vbo, vertex_binding: u32) {
        unsafe {
            gl::VertexArrayVertexBuffer(
                self.id,
                vertex_binding,
                buffer.id(),
                0,
                buffer.vert_size() as i32,
            );
        }
    }

    pub fn vertex_array_attrib_format(&self) {
        unsafe {
            gl::VertexArrayAttribFormat(self.id, );
        }
    }

    pub fn enable(&self) {
        unsafe {
            gl::EnableVertexArrayAttrib(self.id, 0);
        }
    }
}

impl Drop for simple_vao {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}
