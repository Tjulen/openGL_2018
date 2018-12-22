use nalgebra_glm::Vec4;

trait VertexAttribute {
    fn attrib_type() -> gl::types::GLenum;
    fn size() -> u32;
    fn size_of() -> u32;
}
pub trait Vertex {
    fn size_of(&self) -> usize;
    fn attributes() -> [dyn VertexAttribute];
}

impl VertexAttribute for Vec4 {
    #[inline]
    fn attrib_type() -> gl::types::GLenum {
        gl::FLOAT
    }
    #[inline]
    fn size() -> u32 {
        4
    }
    #[inline]
    fn size_of() -> u32 {
        std::mem::size_of::<Self>() as u32
    }
}