#version 450 core

uniform sampler2D s;
in vec2 fTexCoord;

out vec4 color;

void main() {
    color = texture(s, fTexCoord * vec2(3.0, 1.0));
}
