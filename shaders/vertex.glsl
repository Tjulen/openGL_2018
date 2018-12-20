#version 450 core

out vec4 vs_color;

void main() {
    const vec4 vertices[3] = vec4[3] (vec4( 0.25, -0.25, 0.5, 1.0),
                                      vec4(-0.25, -0.25, 0.5, 1.0),
                                      vec4( 0.25,  0.25, 0.5, 1.0));
    const vec4 colors[3] = vec4[3] (
        vec4( 0.25, -0.50, 0.5, 1.0),
        vec4(-0.25, -0.25, 0.1, 1.0),
        vec4( 0.5,  0.25, 0.5, 1.0)
    );
    vs_color = colors[gl_VertexID];
    gl_Position = vertices[gl_VertexID]; //+ offset
}