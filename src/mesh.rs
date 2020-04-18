use std::mem::size_of;
use std::os::raw::c_void;
use std::ptr;

use cgmath::{ Vector3, Vector2 };
use cgmath::prelude::*;
use gl;

use crate::shader::Shader;

pub struct Vertex{
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub tex_coords: Vector2<f32>,
}

impl Default for Vertex{
    fn default() -> Self{
        Vertex{
            position: Vector3::zero(),
            normal: Vector3::zero(),
            tex_coords: Vector2::zero(),
        }
    }
}

#[derive(Clone)]
pub struct Texture{
    pub id: u32,
    pub type_: String,
    pub path: String,
}

pub struct Mesh{
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub textures: Vec<Texture>,
    pub VAO: u32,

    VBO: u32,
    EBO: u32,
}

impl Mesh{
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, textures: Vec<Texture>) -> Mesh{
        let mut mesh = Mesh{
            vertices, indices, textures,
            VAO: 0, VBO: 0, EBO: 0,
        };

        unsafe { mesh.setupMesh() };
        mesh
    }

    pub unsafe fn draw(&self, shader: &Shader){
        let mut diffuseN = 0;
        let mut specularN = 0;

        for (i, texture) in self.textures.iter().enumerate() {
            gl::ActiveTexture(gl::TEXTURE0 + i as u32);

            let name = &texture.type_;
            let number = match name.as_str() {
                "texture_diffuse" => {
                    diffuseN += 1;
                    diffuseN
                },
                "texture_specular" => {
                    specularN += 1;
                    specularN
                },
                _ => panic!("unknown texture type")
            };

            shader.setInt(&format!("material.{}{}", name, number), i as i32);
            gl::BindTexture(gl::TEXTURE_2D, texture.id);
        }

        gl::BindVertexArray(self.VAO);
        gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, ptr::null());
        
        gl::BindVertexArray(0);
        gl::ActiveTexture(gl::TEXTURE0);
    }

    unsafe fn setupMesh(&mut self){
        gl::GenVertexArrays(1, &mut self.VAO);
        gl::GenBuffers(1, &mut self.VBO);
        gl::GenBuffers(1, &mut self.EBO);

        gl::BindVertexArray(self.VAO);

        gl::BindBuffer(gl::ARRAY_BUFFER, self.VBO);
        let size = (self.vertices.len() * size_of::<Vertex>()) as isize;
        let data = &self.vertices[0] as *const Vertex as *const c_void;
        gl::BufferData(gl::ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.EBO);
        let size = (self.indices.len() * size_of::<u32>()) as isize;
        let data = &self.indices[0] as *const u32 as *const c_void;
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

        let size = size_of::<Vertex>() as i32;

        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, size, offset_of!(Vertex, position) as *const c_void);

        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, size, offset_of!(Vertex, normal) as *const c_void);

        gl::EnableVertexAttribArray(2);
        gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, size, offset_of!(Vertex, tex_coords) as *const c_void);

        gl::BindVertexArray(0);
    }

}