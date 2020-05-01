#version 330 core

layout(triangles) in;
layout(triangle_strip, max_vertices = 3) out;

in vec3 g_normal[];
in vec3 g_frag_pos[];
in vec2 g_uv[];
in vec3 g_mvp_normal[];

out vec3 normal;
out vec3 frag_pos;
out vec2 uv;

uniform float time;

 vec4 explode(vec4 position, vec3 normal){
     return position + (sin(time)+1.0) * vec4(normal, 0.0);
 }

 vec3 getNormal(){
     vec3 a = gl_in[0].gl_Position.xyz - gl_in[1].gl_Position.xyz;
     vec3 b = gl_in[2].gl_Position.xyz - gl_in[1].gl_Position.xyz;
     return normalize(cross(a, b));
 }

 void main(){
     vec3 norm = getNormal();

     for(int i = 0; i<3; i++){
         gl_Position = explode(gl_in[i].gl_Position, norm);
         normal = g_normal[i];
         frag_pos = g_frag_pos[i];
         uv = g_uv[i];
         EmitVertex();
     }

     EndPrimitive();

    /* for(int i = 0; i<3; i++){
        normal = g_normal[i];
        frag_pos = g_frag_pos[i];
        uv = g_uv[i];
        gl_Position = gl_in[i].gl_Position;
        EmitVertex();
        gl_Position = gl_in[i].gl_Position + vec4(g_mvp_normal[i], 0.0) * 0.4;
        EmitVertex();
        EndPrimitive();
    } */
 }