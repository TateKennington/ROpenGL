#version 330 core

layout (location = 0) in vec3 pos;
layout (location = 1) in vec3 a_normal;
layout (location = 2) in vec2 a_uv;

out vec3 normal;
out vec3 frag_pos;
out vec2 uv;
out vec3 g_mvp_normal;

layout (std140) uniform Matrices {
    uniform mat4 u_projection;
    uniform mat4 u_view;
};

uniform mat4 u_model;

void main() {
    gl_Position = u_projection * u_view * u_model * vec4(pos.x, pos.y, pos.z, 1.0);
    normal = mat3(transpose(inverse(u_model))) * a_normal;
    g_mvp_normal = normalize(vec3(u_projection * vec4(mat3(inverse(transpose(u_view * u_model))) * a_normal, 1.0)));
    frag_pos = (u_model*vec4(pos, 1.0)).xyz;
    uv = a_uv;
}