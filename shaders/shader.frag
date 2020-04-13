#version 330 core

in vec3 frag_pos;
in vec3 normal;
in vec2 uv;

out vec4 color;

struct Material{
    sampler2D diffuse;
    sampler2D specular;
    sampler2D emission;
    float shininess;
};

struct Light{
    vec3 pos;
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

uniform vec3 camera_pos;
uniform Light light;
uniform Material material;

void main() {
    vec3 ambient = vec3(texture(material.diffuse, uv)) * light.ambient;

    vec3 norm = normalize(normal);
    vec3 light_dir = normalize(light.pos - frag_pos);

    float diff = max(dot(norm, light_dir), 0.0);
    vec3 diffuse = diff*light.diffuse*vec3(texture(material.diffuse, uv));

    vec3 view_dir = normalize(camera_pos - frag_pos);
    vec3 reflect_dir = reflect(-light_dir, norm);

    float spec = pow(max(dot(view_dir, reflect_dir), 0.0), material.shininess);
    vec3 specular = spec * light.specular * vec3(texture(material.specular, uv));

    color = vec4((ambient + diffuse + specular), 1.0) + texture(material.emission, uv);
}