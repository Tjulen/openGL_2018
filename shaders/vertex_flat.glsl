#version 450 core

layout (location = 0) in vec3 vPos;
layout (location = 3) uniform mat4 mv_matrix;
layout (location = 4) uniform mat4 proj_matrix;

void main() {
    vec4 position = vec4(vPos, 1.0);
    gl_Position = proj_matrix * mv_matrix * position;
}