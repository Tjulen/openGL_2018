#[macro_export]
macro_rules! impl_vertex {
    (
        $vertex_name:ident, $($attrib_name:ident: $attrib_index:ident,)+
    ) => {
        impl $vertex_name {
            fn setup_vao(vao_id: gl::types::GLuint) {
                let mut offset = 0;
                unsafe {
                    $(
                        gl::VertexArrayAttribFormat(
                            vao_id,
                            $attrib_index,
                            $attrib_name.size(),
                            $attrib_name.attribute_type(),
                            gl::FALSE,
                            offset,
                        );
                        gl::VertexArrayAttribBinding(vao_id, $attrib_index, 0);
                        gl::EnableVertexArrayAttrib(vao_id, $attrib_index);
                        offset += $attrib_name.size_of();
                    )+;
                }
            }
        }

    };
}
