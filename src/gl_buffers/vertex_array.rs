use crate::gl_vertex::Vertex;
use crate::gl_vertex::VertexAttribute;
use gl::types::*;

pub struct Vao {
    id: GLuint,
    vert_size: i32,
}
impl Vao {
    pub fn new(vert_size: i32) -> Vao {
        let mut id = 0;
        unsafe {
            gl::CreateVertexArrays(1, &mut id);
        }
        Vao { id, vert_size }
    }
    pub fn id(&self) -> GLuint {
        self.id
    }
    pub fn bind_buffer(&self, buffer_id: GLuint) {
        unsafe {
            //the vertex binding is 0, because an array of vertices is my filosophy.
            //Instead of array for each vertex attribute, here we use array of Vertices, that containattributes
            gl::VertexArrayVertexBuffer(self.id, 0, buffer_id, 0, self.vert_size);
        }
    }
    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }
    pub fn attrib_setup(&self, attribs: Vec<Box<VertexAttribute>>) {
        let mut offset = 0;
        let mut count = 0;
        for attrib in attribs {
            unsafe {
                gl::VertexArrayAttribFormat(
                    self.id,
                    count,
                    attrib.size(),
                    attrib.gl_type(),
                    gl::FALSE,
                    offset,
                );
                gl::VertexArrayAttribBinding(self.id, count, 0);
                gl::EnableVertexArrayAttrib(self.id, count);
                offset += attrib.size_of();
                count += 1;
            }
        }
    }
    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}
impl Drop for Vao {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}
