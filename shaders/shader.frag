#version 330 core

out vec4 color;

uniform vec4 light_color;
uniform vec4 object_color;

void main() {
    color = object_color * light_color;
}