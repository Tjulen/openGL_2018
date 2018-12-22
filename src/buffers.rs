use crate::vertex::Vertex;
use gl::types::*;

pub mod vertex_array {
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
                //Instead of array for each vertex attribute, here we use array of Vertices, that contain attributes
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
}

pub struct simple_vbo {
    id: GLuint,
}

impl simple_vbo {
    pub fn from_vec(data: Vec<impl Vertex>) -> simple_vbo {
        let mut vbo_id = 0;
        unsafe {
            gl::CreateBuffers(1, &mut vbo_id);
        }
        let vert_size = data[0].size_of() as usize;
        let simple_vbo = simple_vbo {
            id: vbo_id,
            vert_size,
        };
        unsafe {
            gl::NamedBufferStorage(
                vbo_id,
                (vert_size * data.len()) as isize,
                data.as_ptr() as *const std::ffi::c_void,
                gl::DYNAMIC_STORAGE_BIT,
            );
        }

        simple_vbo
    }
    pub fn id(&self) -> GLuint {
        self.id
    }
    pub fn vert_size(&self) -> usize {
        self.vert_size
    }
}

impl Drop for simple_vbo {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}
