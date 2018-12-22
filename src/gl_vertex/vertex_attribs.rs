use nalgebra_glm::Vec4;


impl crate::gl_vertex::traits::VertexAttribute for Vec4 {
    #[inline]
    fn gl_type(&self) -> gl::types::GLenum {
        gl::FLOAT
    }
    #[inline]
    fn size(&self) -> i32 {
        4
    }
    #[inline]
    fn size_of(&self) -> u32 {
        std::mem::size_of::<Self>() as u32
    }
}