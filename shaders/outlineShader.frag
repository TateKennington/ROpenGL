#version 330 core

in vec3 frag_pos;
in vec3 normal;
in vec2 uv;

out vec4 color;

void main(){
    color = vec4(0.04, 0.28, 0.26, 1.0);
}