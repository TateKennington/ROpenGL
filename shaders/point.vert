#version 330 core

layout (location = 0) in vec3 i_pos;

layout (std140) uniform Matrices{
    uniform mat4 u_proj;
    uniform mat4 u_view;
};
uniform mat4 u_model;

void main(){
    gl_Position = u_proj * u_view * u_model * vec4(i_pos, 1.0);
    gl_PointSize = gl_Position.z;
}