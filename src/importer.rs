use tobj;
use crate::entity::Entity;
use crate::shader::Program;
use crate::gl_buffers::attrib_buffer::AttribBuffer;
use std::path::Path;

pub fn import_entity<'a>(path: &Path, program: &'a Program) -> Entity<'a>{
    let entity = tobj::load_obj(path);
    let (models, _) = entity.unwrap();


    let vertices: Vec<f32> = mesh_to_vertices(&models[0].mesh);
    let mut buffer = AttribBuffer::new("pos".to_string(),gl::FLOAT, 3);
    buffer.array_data(&vertices, gl::STATIC_DRAW);
    Entity::new(program, vec![buffer],models[0].mesh.positions.len() as u64)
}

fn mesh_to_vertices(mesh: &tobj::Mesh) -> Vec<f32> {
    let mut vertices = Vec::new();
    mesh
        .indices
        .iter()
        .for_each(|i| {
            vertices.push(mesh.positions[(i * 3) as usize]);
            vertices.push(mesh.positions[(i * 3 + 1) as usize]);
            vertices.push(mesh.positions[(i * 3 + 2) as usize]);
        });
    vertices
}

