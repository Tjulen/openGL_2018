use gl::types::*;
pub struct Vao {
    id: GLuint,
}
impl Vao {
    pub fn new() -> Vao {
        let mut id = 0;
        unsafe {
            gl::CreateVertexArrays(1, &mut id);
        }
        Vao { id }
    }
    pub fn id(&self) -> GLuint {
        self.id
    }
    pub fn bind_buffer(&self, buffer_id: GLuint, vert_size: i32) {
        unsafe {
            //the vertex binding is 0, because an array of vertices is my filosophy.
            //Instead of array for each vertex attribute, here we use array of Vertices, that containattributes
            gl::VertexArrayVertexBuffer(self.id, 0, buffer_id, 0, vert_size);
        }
    }
    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }
    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}
impl Drop for Vao {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}