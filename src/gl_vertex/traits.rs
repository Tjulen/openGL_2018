pub trait VertexAttribute {
    fn gl_type(&self) -> gl::types::GLenum;
    fn size(&self) -> i32;
    fn size_of(&self) -> u32;
}

pub trait Vertex {
    fn vert_size() -> usize;
    fn attribs() -> Vec<Box<VertexAttribute>> where Self: Sized;
}