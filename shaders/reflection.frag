#version 330 core

in vec3 frag_pos;
in vec3 normal;
in vec2 uv;

out vec4 color;

uniform vec3 camera_pos;
uniform samplerCube skybox;

void main(){
    vec3 i = normalize(frag_pos - camera_pos);
    vec3 r = reflect(i , normalize(normal));
    /* float ratio = 1.00 / 1.52;
    vec3 i = normalize(frag_pos - camera_pos);
    vec3 r = refract(i , normalize(normal), ratio); */
    color = vec4(texture(skybox, r).rgb, 1.0);
}