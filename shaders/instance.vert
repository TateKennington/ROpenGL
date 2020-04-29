#version 330 core
layout(location = 0) in vec3 a_pos;
layout(location = 2) in mat4 a_model;

layout (std140) uniform Matrices {
    uniform mat4 u_proj;
    uniform mat4 u_view;
};

void main(){
    gl_Position = u_proj * u_view * a_model * vec4(a_pos, 1.0);
}