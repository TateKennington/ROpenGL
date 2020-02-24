#version 330 core

layout (location = 0) in vec3 pos;
layout (location = 1) in vec3 a_normal;

out vec3 normal;
out vec3 frag_pos;

uniform mat4 u_model;
uniform mat4 u_view;
uniform mat4 u_projection;

void main() {
    gl_Position = u_projection * u_view * u_model * vec4(pos.x, pos.y, pos.z, 1.0);
    normal = a_normal;
    frag_pos = (u_model*vec4(pos, 1.0)).xyz;
}