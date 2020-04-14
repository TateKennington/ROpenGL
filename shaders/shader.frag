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
    vec4 light_vector;

    vec3 direction;
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;

    float c;
    float l;
    float q;

    float cutoff;
    float outerCutoff;
};

uniform vec3 camera_pos;
uniform Light light;
uniform Material material;

void main() {

    vec3 norm = normalize(normal);
    vec3 light_dir;
    float attenuation = 1.0;

    if (light.light_vector.w == 1.0){
        vec3 light_pos = light.light_vector.xyz;
        light_dir = normalize(light_pos - frag_pos);

        float d = length(light_pos - frag_pos);
        attenuation = 1/(light.c + light.l * d + light.q * d * d);
    } else {
        light_dir = normalize(-light.light_vector.xyz);
    }

    if(light.cutoff != -1.0){
        float theta = dot(light_dir, normalize(-light.direction));
        attenuation *= clamp((theta - light.outerCutoff)/(light.cutoff - light.outerCutoff), 0.0, 1.0);
    }

    vec3 ambient = vec3(texture(material.diffuse, uv)) * light.ambient;

    float diff = max(dot(norm, light_dir), 0.0);
    vec3 diffuse = diff*light.diffuse*vec3(texture(material.diffuse, uv));

    vec3 view_dir = normalize(camera_pos - frag_pos);
    vec3 reflect_dir = reflect(-light_dir, norm);

    float spec = pow(max(dot(view_dir, reflect_dir), 0.0), material.shininess);
    vec3 specular = spec * light.specular * vec3(texture(material.specular, uv));

    color = vec4(attenuation*(ambient + diffuse + specular), 1.0)/*  + texture(material.emission, uv) */;
}