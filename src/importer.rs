use crate::entity::Entity;
use crate::gl_buffers::AttribBuffer;
use crate::shader::Program;
use glm::vec3;
use glm::Vec3;
use glm::Vec2;
use glm::vec2;
use std::path::Path;
use tobj;

pub fn import_entity<'a>(path: &Path, program: &'a Program) -> Entity<'a> {
    let entity = tobj::load_obj(path);
    let (models, _) = entity.unwrap();

    let vertices: Vec<Vec3> = mesh_to_vertices(&models[0].mesh);
    let tex_coords: Vec<Vec2> = mesh_to_tex_coord(&models[0].mesh);
    let mut vert_buffer = AttribBuffer::new("vPos".to_string(), gl::FLOAT, 3);
    let mut tex_coords_buffer = AttribBuffer::new("vTexCoord".to_string(), gl::FLOAT, 2);
    vert_buffer.array_data(&vertices, gl::STATIC_DRAW);
    tex_coords_buffer.array_data(&tex_coords, gl::STATIC_DRAW);
    Entity::new(program, vec![vert_buffer, tex_coords_buffer], vertices.len() as u64)
}

fn mesh_to_tex_coord(mesh: &tobj::Mesh) -> Vec<Vec2> {
    let mut tex_coords = Vec::new();
    for idx in 0..mesh.texcoords.len() / 2 {
        tex_coords.push(
            vec2(mesh.texcoords[2 * idx], mesh.texcoords[2 * idx + 1])
        )
    }
    tex_coords
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
