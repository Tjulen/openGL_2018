use nalgebra_glm::Vec4;

pub trait VertexAttribute {
    fn gl_type(&self) -> gl::types::GLenum;
    fn size(&self) -> i32;
    fn size_of(&self) -> u32;
}
pub trait Vertex {
    fn vert_size() -> usize;
    fn attribs() -> Vec<Box<VertexAttribute>> where Self: Sized;
}

impl VertexAttribute for Vec4 {
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