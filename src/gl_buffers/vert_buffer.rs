use crate::gl::types::*;

#[derive(Debug)]
pub struct Buffer {
    pub id: GLuint,
    pub vert_len: i32,
    pub shader_binding: i8,
    pub vao_binding: i8,
    pub num: i8,
    pub ty: GLenum,
    pub size: i32,
}
impl Buffer {
    pub fn new(shader_binding: i8, vao_binding: i8, ty: GLenum, num: i8) -> Buffer {
        let mut id = 0;
        unsafe {
            gl::CreateBuffers(1, &mut id);
        }
        Buffer { id, vert_len: 0, shader_binding, vao_binding, ty, num, size: 0 }
    }
    pub fn vec_data<T: Sized>(&mut self, data: &Vec<T>, usage_flags: GLenum) {
        unsafe {
            gl::NamedBufferData(
                self.id,
                (std::mem::size_of::<T>() * data.len()) as isize,
                data.as_ptr() as *const std::ffi::c_void,
                usage_flags,
            );
            self.size = std::mem::size_of::<T>() as i32;
            self.vert_len = data.len() as i32;
        }
    }
    pub fn array_data<T: Sized>(&mut self, data: &[T], usage_flags: GLenum) {
        unsafe {
            //Data instead of storage, beacause storage is immutable and i use it's mutability for now (less performance)
            //TODO: implement so that it uses Storage, but uses maping for getting the information to the GPU
            gl::NamedBufferData(
                self.id,
                (std::mem::size_of::<T>() * data.len()) as isize,
                data.as_ptr() as *const std::ffi::c_void,
                usage_flags,
            );
            self.size = std::mem::size_of::<T>() as i32;
            self.vert_len = data.len() as i32;
        }
    }
}
impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}
