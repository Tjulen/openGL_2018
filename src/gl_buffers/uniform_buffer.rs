use gl::types::*;
use glm::*;

//TODO use references to types, so no need to constantlly update data with uniformBuffer.data = newData...
pub enum UniformType {
    Float(f32),
    Vec2(Vec2),
    Vec3(Vec3),
    Vec4(Vec4),
    Matrix4(Mat4),
}

pub struct UniformBuffer {
    //Automatically assigned when given to entity (it look for the name in shader and returns the shader_binding num)
    pub shader_binding: u8,
    pub ty: UniformType,
}

impl UniformBuffer {
    pub fn new(shader_binding: u8, ty: UniformType) -> UniformBuffer {
        UniformBuffer {
            shader_binding,
            ty,
        }
    }
    pub fn data(&mut self, data: UniformType) {
        self.ty = data;
    }
    #[inline]
    pub fn bind(&self) {
        match self.ty {
            UniformType::Float(data) => unsafe {
                gl::Uniform1f(self.shader_binding as GLint, data as GLfloat);
            },
            UniformType::Vec2(data) => unsafe {
                gl::Uniform2fv(
                    self.shader_binding as GLint,
                    2,
                    data.as_slice().as_ptr() as *const GLfloat,
                )
            },
            UniformType::Vec3(data) => unsafe {
                gl::Uniform2fv(
                    self.shader_binding as GLint,
                    3,
                    data.as_slice().as_ptr() as *const GLfloat,
                )
            },
            UniformType::Vec4(data) => unsafe {
                gl::Uniform2fv(
                    self.shader_binding as GLint,
                    4,
                    data.as_slice().as_ptr() as *const GLfloat,
                )
            },
            UniformType::Matrix4(data) => unsafe {
                gl::UniformMatrix4fv(
                    self.shader_binding as GLint,
                    1,
                    gl::FALSE,
                    data.as_slice().as_ptr() as *const GLfloat,
                )
            }
        }
    }
}
