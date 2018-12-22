use nalgebra_glm::Vec4;

trait Attribute {
    fn attrib_type() -> gl::types::GLenum;
    fn size() -> u32;
    fn size_of() -> u32;
}
pub trait Vertex {
    fn size_of(&self) -> usize;
}

impl Attribute for Vec4 {
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
        std::mem::size_of::<Self>()
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
struct simple_vertex {
    pos: Vec4,
}

impl Vertex for simple_vertex {
    #[inline]
    fn size_of(&self) -> usize {
        std::mem::size_of::<Self>()
    }
}