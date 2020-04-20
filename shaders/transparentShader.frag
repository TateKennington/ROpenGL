#version 330 core

in vec2 uv;

out vec4 color;

uniform sampler2D texture1;


void main(){
    color = texture(texture1, uv);
    if (color.a < 0.1){
        discard;
    }
}