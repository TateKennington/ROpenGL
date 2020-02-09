#version 330 core

layout (location = 0) in vec3 pos;
layout (location = 1) in vec3 color;
layout (location = 2) in vec2 i_texCoords;

out vec3 vert_color;
out vec2 texCoords;

void main() {
    gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
    vert_color = color;
    texCoords = i_texCoords;
}