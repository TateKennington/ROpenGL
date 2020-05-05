#version 330 core

layout(location = 0) in vec3 a_pos;

uniform mat4 u_model;
uniform mat4 lightspace_transform;

void main(){
    gl_Position = lightspace_transform * u_model * vec4(a_pos, 1.0);
}