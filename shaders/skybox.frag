#version 330 core

in vec3 tex_coords;

out vec4 color;

uniform samplerCube skybox;

void main(){
    color = texture(skybox, tex_coords);
    //color = vec4(tex_coords, 1.0);
}