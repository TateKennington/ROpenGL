#version 330 core

layout (location = 0) in vec3 pos;
layout (location = 1) in vec2 a_uv;

out vec3 frag_pos;
out vec2 uv;

uniform mat4 u_model;
uniform mat4 u_view;
uniform mat4 u_projection;

void main() {
    gl_Position = u_projection * u_view * u_model * vec4(pos.x, pos.y, pos.z, 1.0);
    frag_pos = (u_model*vec4(pos, 1.0)).xyz;
    uv = a_uv;
}