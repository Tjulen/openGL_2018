use super::attrib_buffer::AttribBuffer;
use gl::types::*;

#[derive(Debug)]
pub struct VertexArray {
    pub id: GLuint,
    //INFO only 10 buffers per vao are allowed, if you need more change this number (but not sure how many bindings the vao supports)
    //INFO used to assign buffers to vao bindings procedurally
    //TODO find a better way to do this
    vao_bindings_count: i8,
}
impl VertexArray {
    pub fn new() -> VertexArray {
        let mut id = 0;
        unsafe {
            gl::CreateVertexArrays(1, &mut id);
        }
        VertexArray { id, vao_bindings_count: 0 }
    }
    #[inline]
    pub fn setup_attrib(&mut self, buffer: &mut AttribBuffer) {
        self.assign_vao_binding(buffer);
        unsafe {
            gl::VertexArrayAttribBinding(
                self.id,
                buffer.vao_binding as u32,
                buffer.shader_binding as u32,
            );
            gl::EnableVertexArrayAttrib(self.id, buffer.vao_binding as u32);
            gl::VertexArrayAttribFormat(
                self.id,
                buffer.vao_binding as u32,
                i32::from(buffer.num),
                buffer.ty,
                gl::FALSE,
                0,
            );
        }
    }
    fn assign_vao_binding(&mut self, buffer: &mut AttribBuffer) {
        if self.vao_bindings_count > 10 {
            panic!("More than 10 buffers assigned to vao!");
        }
        buffer.vao_binding = self.vao_bindings_count;
        self.vao_bindings_count += 1;
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
    #[inline]
    pub fn bind_buffer(&self, buffer: &AttribBuffer) {
        unsafe {
            gl::VertexArrayVertexBuffer(
                self.id,
                buffer.vao_binding as u32,
                buffer.id,
                0 as GLintptr,
                buffer.size
            );
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
