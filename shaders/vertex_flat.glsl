#version 450 core

layout (location = 0) in vec3 vPos;
layout (location = 3) uniform mat4 mv_matrix;
layout (location = 4) uniform mat4 proj_matrix;

out vec3 fCol;

void main() {
    vec4 position = vec4(vPos, 1.0);
    fCol = vPos * 2.0 + vec3(0.5, 0.5, 0.5);
    gl_Position = proj_matrix * mv_matrix * position;

}