use nalgebra::Vector4;

trait Attribute {
    fn type() -> AttributeType;
    fn size() -> u32;
}
pub trait Vertex {
    fn size_of(&self) -> usize;
}

impl Attribute for Vector4<i32> {
    #[inline]
    fn type() -> gl::Enum {
        gl::INT
    }
    #[inline]
    fn size() -> u32 {
        4
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
struct simple_vertex {
    pos: Vector4<i32>,
}

impl_vertex!(simple_vertex, pos);

impl Vertex for simple_vertex {
    #[inline]
    fn size_of(&self) -> usize {
        std::mem::size_of::<Self>()
    }
}