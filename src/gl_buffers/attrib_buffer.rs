use gl::types::*;


pub struct AttribBuffer {
    pub id: GLuint,
    //Automatically assigned when given to entity (it look for the name in shader and returns the shader_binding num)
    pub shader_binding: i8,
    //Assigned when given to Vao
    pub vao_binding: i8,
    pub num: i8,
    pub ty: GLenum,
    pub size: i32,
    pub name: String
}

impl AttribBuffer {
    pub fn new(attrib_name: String, ty: GLenum, num: i8) -> AttribBuffer{
        let mut id = 0;
        unsafe {
            gl::CreateBuffers(1, &mut id);
        }
        AttribBuffer {
            id,
            shader_binding: -1,
            vao_binding: -1,
            num,
            ty,
            size: -1,
            name: attrib_name
        }
    }
    pub fn array_data<T: Sized>(&mut self, data: &[T], usage_flags: GLenum) {
        unsafe {
            //INFO Data instead of storage, because storage is immutable and i use it's mutability for now (less performance)
            //TODO implement so that it uses Storage, but uses mapping for getting the information to the GPU (less copying around)
            gl::NamedBufferData(
                self.id,
                (std::mem::size_of::<T>() * data.len()) as isize,
                data.as_ptr() as *const std::ffi::c_void,
                usage_flags,
            );
            self.size = std::mem::size_of::<T>() as i32;
        }
    }
}

impl Drop for AttribBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}