#version 330 core

out vec4 color;

in vec3 vert_color;
in vec2 texCoords;

uniform vec4 u_color;
uniform float u_mix_param;
uniform sampler2D tex0;
uniform sampler2D tex1;

void main() {
    color = mix(texture(tex0, texCoords), texture(tex1, 2*texCoords*vec2(-1, 1)), u_mix_param);
}