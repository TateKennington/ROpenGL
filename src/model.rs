use std::os::raw::c_void;
use std::path::Path;

use cgmath::{vec2, vec3};
use gl;
use image;
use image::DynamicImage::*;
use image::GenericImageView;
use tobj;

use crate::mesh::{ Mesh, Texture, Vertex };
use crate::shader::Shader;

#[derive(Default)]
pub struct Model {
    pub meshes: Vec<Mesh>,
    pub textures_loaded: Vec<Texture>,
    directory: String,
}

impl Model {
    pub fn new(path: &str) -> Model{
        let mut model = Model::default();
        model.loadModel(path);
        model
    }

    pub fn draw(&self, shader: &Shader){
        for mesh in &self.meshes{
            unsafe { mesh.draw(shader); }
        }
    }

    fn loadModel(&mut self, path: &str){
        let path = Path::new(path);

        self.directory = path.parent().unwrap_or_else(|| Path::new("")).to_str().unwrap().into();
        let obj = tobj::load_obj(path);

        let (models, materials) = obj.unwrap();
        for model in models{
            let mesh = &model.mesh;
            let num_vertices = mesh.positions.len()/3;

            let mut vertices = Vec::<Vertex>::with_capacity(num_vertices);
            let indices: Vec<u32> = mesh.indices.clone();
            
            let (p, n, t) = (&mesh.positions, &mesh.normals, &mesh.texcoords);
            for i in 0..num_vertices{
                vertices.push(Vertex{
                    position: vec3(p[i*3], p[i*3+1], p[i*3+2]),
                    normal: vec3(n[i*3], n[i*3+1], n[i*3+2]),
                    tex_coords: vec2(t[i*2], t[i*2+1]),
                    ..Vertex::default()
                })
            }

            let mut textures = Vec::<Texture>::new();
            if let Some(material_id) = mesh.material_id {
                let material = &materials[material_id];

                if !material.diffuse_texture.is_empty() {
                    let texture = self.loadMaterialTexture(&material.diffuse_texture, "texture_diffuse");
                    textures.push(texture);
                }

                if !material.specular_texture.is_empty() {
                    let texture = self.loadMaterialTexture(&material.specular_texture, "texture_specular");
                    textures.push(texture);
                }
            }
            self.meshes.push(Mesh::new(vertices, indices, textures));
        }
    }

    fn loadMaterialTexture(&mut self, path: &str, typeName: &str) -> Texture{
        {
            let texture = self.textures_loaded.iter().find(|t| t.path == path);
            if let Some(texture) = texture{
                return texture.clone();
            }
        }

        let id = unsafe {
            let filename = format!("{}/{}", self.directory, path);

            let mut texture_id = 0;
            gl::GenTextures(1, &mut texture_id);

            let img = image::open(&Path::new(&filename)).expect(&format!("Failed to load texture: {}", filename));
            let img = img.flipv();
            let (format, internalFormat) = match img{
                ImageLuma8(_) => (gl::RED, gl::RED),
                ImageLumaA8(_) => (gl::RG, gl::RG),
                ImageRgb8(_) => (gl::SRGB, gl::RGB),
                ImageRgba8(_) => (gl::SRGB_ALPHA, gl::RGBA),
                _ => panic!("Unsupported image format")
            };

            let data = img.raw_pixels();

            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            gl::TexImage2D(gl::TEXTURE_2D, 0, format as i32, img.width() as i32, img.height() as i32,
                            0, internalFormat, gl::UNSIGNED_BYTE, &data[0] as *const u8 as *const c_void);
            gl::GenerateMipmap(gl::TEXTURE_2D);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            texture_id
        };

        let texture = Texture {
            id,
            type_: typeName.into(),
            path: path.into()
        };

        self.textures_loaded.push(texture.clone());
        texture
    }
}