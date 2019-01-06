use super::vert_buffer::Buffer;
use gl::types::*;

#[derive(Debug)]
pub struct VertexArray {
    pub id: GLuint,
}
impl VertexArray {
    pub fn new() -> VertexArray {
        let mut id = 0;
        unsafe {
            gl::CreateVertexArrays(1, &mut id);
        }
        VertexArray { id }
    }
    #[inline]
    pub fn setup_attrib(&self, buffer: &Buffer) {
        unsafe {
            gl::VertexArrayAttribBinding(self.id, buffer.vao_binding as u32, buffer.shader_binding as u32);
            gl::EnableVertexArrayAttrib(self.id, buffer.vao_binding as u32);
            gl::VertexArrayAttribFormat(self.id, buffer.vao_binding as u32, i32::from(buffer.num), buffer.ty, gl::FALSE, 0);
        }
    }
    #[inline]
    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }
    #[inline]
    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}
impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}
