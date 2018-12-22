#[macro_export]
macro_rules! impl_vertex {
    (
        $vertex_name:ident, $($attrib_name:ident,)+
    ) => {
        impl $vertex_name {
            fn create_vao(buffer: Buffer) -> VertexArray {
                let mut vao_id: GLuint = 0;
                let mut count = 0;
                unsafe {
                    gl::CreateVertexArrays(1, &mut vao_id);
                    gl::VertexArrayVertexBuffer(
                        vao_id,
                        0,
                        buffer.id,
                        0,
                        std::mem::size_of::<Self>() as i32,
                    );
                }

                VertexArray {
                    id: vao_id,
                    vbo: buffer,
                }
            }
            fn set_vertex_attrib_format() {
                let mut offset: usize = 0;
                let mut count = 0;
                unsafe {
                    $(
                        gl::VertexAttribFormat(count, $size, $type, gl::FALSE, offset as u32);
                        offset += std::mem::size_of::<$field_type>();
                        count += 1;
                    )+;
                }
            }
        }

    };
}
