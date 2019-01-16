#version 450 core

in vec4 vPos;
in vec4 vCol;

out vec4 vs_color;

void main() {
    vs_color = vCol;
    gl_Position = vPos;
}