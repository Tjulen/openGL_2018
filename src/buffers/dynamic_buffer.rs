use gl::types::*;

pub struct DynamicBuffer {
    id: GLuint,
    size: usize,
}
impl DynamicBuffer {
    pub fn new() -> DynamicBuffer {
        let mut id = 0;
        unsafe {
            gl::CreateBuffers(1, &mut id);
        }
        DynamicBuffer {id, size: 0}
    }
    pub fn vec_data<T: Sized>(&mut self, data: Vec<T>) {
        unsafe {
            gl::NamedBufferStorage(
                self.id,
                (std::mem::size_of::<T>() * data.len()) as isize,
                data.as_ptr() as *const std::ffi::c_void,
                gl::DYNAMIC_STORAGE_BIT,
            );
            self.size = data.len();
        }
    }
    pub fn id(&self) -> GLuint {
        self.id
    }
}
impl Drop for DynamicBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}