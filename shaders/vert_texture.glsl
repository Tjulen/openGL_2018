#version 450 core

in vec3 vPos;
in vec2 vTexCoord;

out vec2 fTexCoord;

void main() {
    fTexCoord = vTexCoord;
    gl_Position = vec4(vPos, 1.0);
}