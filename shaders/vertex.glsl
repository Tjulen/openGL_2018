#version 450 core

layout (location = 0) in vec4 pos;
layout (location = 1) in vec4 col;

out vec4 vs_color;

void main() {
    vs_color = col;
    gl_Position = pos;
}