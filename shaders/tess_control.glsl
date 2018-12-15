#version 450 core

layout (vertices = 3) out;

void main() {
    gl_TessLevelInner[0] = 5.0;
    gl_TessLevelOuter[0] = 5.0;
    gl_TessLevelOuter[1] = 5.0;
    gl_TessLevelOuter[2] = 5.0;

    gl_out[gl_InvocationID].gl_Position = gl_in[gl_InvocationID].gl_Position;
}