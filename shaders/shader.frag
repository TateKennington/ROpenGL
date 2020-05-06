#version 330 core

in vec3 frag_pos;
in vec3 normal;
in vec2 uv;

out vec4 color;

uniform mat4 lightspace_transform;
uniform sampler2D shadow_map;

struct Material{
    sampler2D texture_diffuse1;
    sampler2D texture_specular1;
    sampler2D emission;
    float shininess;
};

struct PointLight{
    vec3 pos;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;

    float c;
    float l;
    float q;
};

struct DirLight {
    vec3 direction;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

struct SpotLight {
    vec3 pos;
    vec3 direction;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;

    float cutoff;
    float outerCutoff;
};

uniform vec3 camera_pos;
uniform DirLight dir_light;
uniform SpotLight spot_light;
uniform PointLight point_lights[6];
uniform Material material;

vec3 calculateDirLight(DirLight dir_light, vec3 norm, vec3 view_dir, vec3 frag_pos){
    vec3 light_dir = normalize(-dir_light.direction);
    vec4 lightspace_pos = lightspace_transform * vec4(frag_pos, 1.0);
    vec3 lightspace_coords = lightspace_pos.xyz / lightspace_pos.w;
    lightspace_coords = 0.5 * lightspace_coords + 0.5;
    //float closest_depth = texture(shadow_map, lightspace_coords.xy).r;
    float current_depth = lightspace_coords.z;
    //float shadow = current_depth - max(0.05* (1.0-dot(norm, light_dir)), 0.005) > closest_depth? 1.0: 0.0;
    float shadow = 0.0;
    vec2 size = 1.0/textureSize(shadow_map, 0);

    for(int x = -1; x<=1; x++){
        for(int y = -1; y<=1; y++){
            float closest_depth = texture(shadow_map, lightspace_coords.xy + vec2(x, y) * size).r;
            shadow += current_depth - 0.005 > closest_depth? 1.0: 0.0;
        }
    }

    shadow/=9.0;
    
    if (lightspace_coords.z > 1.0){
        shadow = 0.0;
    }

    vec3 ambient = vec3(texture(material.texture_diffuse1, uv)) * dir_light.ambient;

    float diff = max(dot(norm, light_dir), 0.0);
    vec3 diffuse = diff*dir_light.diffuse*vec3(texture(material.texture_diffuse1, uv));

    vec3 halfway = normalize(view_dir + light_dir);

    float spec = pow(max(dot(norm, halfway), 0.0), material.shininess);
    vec3 specular = spec * dir_light.specular * vec3(texture(material.texture_specular1, uv));

    return (1.0 - shadow) * (ambient + diffuse + specular);
    //return vec3(texture(shadow_map, lightspace_coords.xy).r);
}

vec3 calculatePointLight(PointLight point_light, vec3 norm, vec3 view_dir, vec3 frag_pos){
    vec3 light_dir = normalize(point_light.pos - frag_pos);
    float d = length(point_light.pos - frag_pos);
    float attenuation = 1/(point_light.c + point_light.l * d + point_light.q * d * d);

    vec3 ambient = vec3(texture(material.texture_diffuse1, uv)) * point_light.ambient;

    float diff = max(dot(norm, light_dir), 0.0);
    vec3 diffuse = diff*point_light.diffuse * vec3(texture(material.texture_diffuse1, uv));

    vec3 halfway = normalize(view_dir + light_dir);

    float spec = pow(max(dot(norm, halfway), 0.0), material.shininess);
    vec3 specular = spec * point_light.specular * vec3(texture(material.texture_specular1, uv));

    return attenuation * (ambient + diffuse + specular);
}

vec3 calculateSpotLight(SpotLight spot_light, vec3 norm, vec3 view_dir, vec3 frag_pos){
    vec3 light_dir = normalize(spot_light.pos - frag_pos);

    float theta = dot(normalize(-spot_light.direction),light_dir);
    float attenuation = clamp((theta - spot_light.outerCutoff)/(spot_light.cutoff - spot_light.outerCutoff), 0.0, 1.0);

    vec3 ambient = vec3(texture(material.texture_diffuse1, uv)) * spot_light.ambient;

    float diff = max(dot(norm, light_dir), 0.0);
    vec3 diffuse = diff*spot_light.diffuse * vec3(texture(material.texture_diffuse1, uv));

    vec3 halfway = normalize(view_dir + light_dir);

    float spec = pow(max(dot(norm, halfway), 0.0), material.shininess);
    vec3 specular = spec * spot_light.specular * vec3(texture(material.texture_specular1, uv));

    return attenuation * ( ambient + diffuse + specular );
}

void main() {
    vec3 norm = normalize(normal);
    vec3 view_dir = normalize(camera_pos - frag_pos);
    vec3 res = vec3(0.0);
    res += calculateDirLight(dir_light, norm, view_dir, frag_pos);

    for(int i = 0; i<6; i++){
       //res += calculatePointLight(point_lights[i], norm, view_dir, frag_pos);
    }

    //res+= calculateSpotLight(spot_light, norm, view_dir, frag_pos);
    color = vec4(res, 1.0);
}