use crate::entity::Entity;
use crate::gl_buffers::AttribBuffer;
use crate::shader::Program;
use glm::vec3;
use glm::Vec3;
use std::path::Path;
use tobj;

pub fn import_entity<'a>(path: &Path, program: &'a Program) -> Entity<'a> {
    let entity = tobj::load_obj(path);
    let (models, _) = entity.unwrap();

    let vertices: Vec<Vec3> = mesh_to_vertices(&models[0].mesh);
    let mut buffer = AttribBuffer::new("vPos".to_string(), gl::FLOAT, 3);
    buffer.array_data(&vertices, gl::STATIC_DRAW);
    Entity::new(program, vec![buffer], vertices.len() as u64)
}

fn mesh_to_vertices(mesh: &tobj::Mesh) -> Vec<Vec3> {
    let mut vertices = Vec::new();
    for idx in &mesh.indices {
        let i = *idx as usize;
        vertices.push(vec3(
            mesh.positions[3 * i],
            mesh.positions[3 * i + 1],
            mesh.positions[3 * i + 2],
        ));
    }
    vertices
}
