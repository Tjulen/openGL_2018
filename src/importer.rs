use assimp::Importer;
use crate::entity::Entity;
use crate::gl_buffers::vert_buffer::Buffer;
use crate::shader::Program;

pub fn import_entity(path: &str, program: Program) -> Entity {
    let mut importer = Importer::new();
    importer.triangulate(true);

    //TODO: better error handling - return Result
    let scene = importer
        .read_file(path)
        .expect(stringify!("ERROR opening the file: {}", path));
    
    let buffers: Vec<Buffer>;
    for mesh in scene.mesh_iter() {
        let mut verts: Vec<f32>;
        mesh.vertex_iter().for_each(|vertex| {
            verts.push(vertex.x);
            verts.push(vertex.y);
            verts.push(vertex.z);
        });
        buffers.push(
            {
                let buffer = Buffer::new();
                buffer.vec_data(&verts, gl::STATIC_DRAW);
                buffer
            }
        );
    }
    Entity::new(program, )
}

