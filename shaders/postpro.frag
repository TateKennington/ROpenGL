#version 330 core

in vec2 uv;

out vec4 color;

uniform sampler2D texture1;

void main(){
    color = vec4(texture(texture1, uv).rgb, 1.0);
}