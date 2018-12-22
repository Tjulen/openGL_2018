use nalgebra::Vector4;

trait Attribute {
    fn get_type() -> AttributeType;
}
pub trait Vertex {
    fn size_of(&self) -> usize;
}

impl Attribute for Vector4<i32> {
    fn get_type() -> gl::Enum {
        gl::INT
    }
}



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