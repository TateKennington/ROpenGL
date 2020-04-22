#version 330 core

in vec2 uv;

out vec4 color;

uniform sampler2D texture1;

void main(){
    color = vec4(texture(texture1, uv).rgb, 1.0);
    //color = vec4(vec3(1.0 - texture(texture1, uv)), 1.0);
    /* float offset = 1.0/300.0;

    vec2 offsets[9] = vec2[](
        vec2(-offset, offset), // top-left
        vec2( 0.0f, offset), // top-center
        vec2( offset, offset), // top-right
        vec2(-offset, 0.0f), // center-left
        vec2( 0.0f, 0.0f), // center-center
        vec2( offset, 0.0f), // center-right
        vec2(-offset, -offset), // bottom-left
        vec2( 0.0f, -offset), // bottom-center
        vec2( offset, -offset) // bottom-right
    );
    float kernel[9] = float[](
        1, 1, 1,
        1, -8, 1,
        1, 1, 1
    );

    vec3 sample[9];
    for(int i = 0; i<9; i++){
        sample[i] = kernel[i] * texture(texture1, uv + offsets[i]).rgb;
    }

    vec3 col = vec3(0.0);
    for(int i = 0; i<9; i++){
        col += sample[i];
    }

    color = vec4(col, 1.0); */
}