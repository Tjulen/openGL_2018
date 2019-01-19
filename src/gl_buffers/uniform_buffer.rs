use gl::types::*;

pub struct UniformBuffer<T> {
    pub data: T,
    //Automatically assigned when given to entity (it look for the name in shader and returns the shader_binding num)
    pub shader_binding: u8,
    pub num: u8,
}

impl<T> UniformBuffer<T> {
    pub fn new(data: T, shader_binding: u8, num: u8) -> UniformBuffer<T> {
        UniformBuffer {
            data,
            shader_binding,
            num
        }
    }
    pub fn data(&mut self, data: T) {
        self.data = data;
    }
}