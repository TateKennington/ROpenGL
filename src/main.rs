extern crate glfw;
extern crate gl;

use glfw::{Context, Key, Action};
use gl::types::*;

use cgmath::{Matrix4, vec3, Deg, perspective, Point3, Vector3};
use cgmath::prelude::*;

use image::GenericImageView;

use std::sync::mpsc::Receiver;
use std::ffi::CString;
use std::ptr;
use std::str;
use std::mem;
use std::os::raw::c_void;
use std::fs::File;
use std::io::Read;
use std::path::Path;

struct Shader{
    id: u32,
}

impl Shader{
    pub fn new(vertexPath: &str, fragmentPath: &str) -> Shader{
        let mut res = Shader{ id:0 };

        let mut vertexShaderSourceFile = File::open(vertexPath).unwrap();
        let mut fragmentShaderSourceFile = File::open(fragmentPath).unwrap();

        let mut vertexShaderSource = String::new();
        let mut fragmentShaderSource = String::new();
        
        vertexShaderSourceFile.read_to_string(&mut vertexShaderSource).unwrap();
        fragmentShaderSourceFile.read_to_string(&mut fragmentShaderSource).unwrap();

        let vertexShaderSource = CString::new(vertexShaderSource.as_bytes()).unwrap();
        let fragmentShaderSource = CString::new(fragmentShaderSource.as_bytes()).unwrap();

        unsafe {
            let vertexShader = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertexShader, 1, &vertexShaderSource.as_ptr(), ptr::null());
            gl::CompileShader(vertexShader);

            let mut success = gl::FALSE as GLint;
            let mut infoLog = Vec::with_capacity(512);
            infoLog.set_len(512-1);
            gl::GetShaderiv(vertexShader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(vertexShader, 512, ptr::null_mut(), infoLog.as_mut_ptr() as *mut GLchar);
                println!("{}", str::from_utf8_unchecked(&infoLog));
            }

            let fragmentShader = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragmentShader, 1, &fragmentShaderSource.as_ptr(), ptr::null());
            gl::CompileShader(fragmentShader);

            gl::GetShaderiv(fragmentShader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(fragmentShader, 512, ptr::null_mut(), infoLog.as_mut_ptr() as *mut GLchar);
                println!("{}", str::from_utf8_unchecked(&infoLog));
            }

            let shaderProgram = gl::CreateProgram();
            gl::AttachShader(shaderProgram, vertexShader);
            gl::AttachShader(shaderProgram, fragmentShader);
            gl::LinkProgram(shaderProgram);

            gl::DeleteShader(vertexShader);
            gl::DeleteShader(fragmentShader);

            res.id = shaderProgram;
        }

        res
    }

    pub fn useProgram(&self){
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn setUniform4f(&self, name: &str, vector: (f32, f32, f32, f32)){
        let name = CString::new(name.as_bytes()).unwrap();
        unsafe{
            gl::Uniform4f(gl::GetUniformLocation(self.id, name.as_ptr()), vector.0, vector.1, vector.2, vector.3,);
        }
    }

    pub fn setInt(&self, name: &str, value: i32){
        let name = CString::new(name.as_bytes()).unwrap();
        unsafe{
            gl::Uniform1i(gl::GetUniformLocation(self.id, name.as_ptr()), value);
        }
    }
    
    pub fn setFloat(&self, name: &str, value: f32){
        let name = CString::new(name.as_bytes()).unwrap();
        unsafe{
            gl::Uniform1f(gl::GetUniformLocation(self.id, name.as_ptr()), value);
        }
    }

    pub fn setMat4(&self, name: &str, value: Matrix4<f32>){
        let name = CString::new(name.as_bytes()).unwrap();
        unsafe{
            gl::UniformMatrix4fv(gl::GetUniformLocation(self.id, name.as_ptr()), 1, gl::FALSE, value.as_ptr());
        }
    }
}

fn main(){
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    let (mut window, events) = glfw.create_window(800, 600, "Slugma", glfw::WindowMode::Windowed)
        .expect("Failed to create glfw window");
    
    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let mut cam_pos = Point3::new(0.0, 0.0, 0.0);
    let mut cam_front = vec3(0.0, 0.0, -1.0);

    let (shaderProgram, VAO, texture0, texture1) = unsafe {

        gl::Enable(gl::DEPTH_TEST);

        let vertices: [f32;32] = [
            0.5, 0.5, 0.0,  1.0, 0.0, 0.0,  1.0, 1.0,
            0.5, -0.5, 0.0,  0.0, 1.0, 0.0,  1.0, 0.0,
            -0.5, -0.5, 0.0,  0.0, 0.0, 1.0,  0.0, 0.0,
            -0.5, 0.5, 0.0,  0.0, 0.0, 0.0,  0.0, 1.0
        ];
        let indices = [
            0, 1, 3,
            1, 2, 3
        ];

        let (mut VBO, mut VAO, mut EBO) = (0, 0, 0);
        gl::GenVertexArrays(1, &mut VAO);
        gl::GenBuffers(1, &mut VBO);
        gl::GenBuffers(1, &mut EBO);

        gl::BindVertexArray(VAO);
        
        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
        gl::BufferData(gl::ARRAY_BUFFER,
                        (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                        &vertices[0] as *const f32 as *const c_void,
                        gl::STATIC_DRAW);
        
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, EBO);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                      (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                      &indices[0] as *const i32 as *const c_void,
                      gl::STATIC_DRAW);

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 8 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 8 * mem::size_of::<GLfloat>() as GLsizei, (3 * mem::size_of::<GLfloat>()) as *const c_void);
        gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, 8 * mem::size_of::<GLfloat>() as GLsizei, (6 * mem::size_of::<GLfloat>()) as *const c_void);
        gl::EnableVertexAttribArray(0);
        gl::EnableVertexAttribArray(1);
        gl::EnableVertexAttribArray(2);

        let mut texture0 = 0;
        gl::GenTextures(1, &mut texture0);
        gl::BindTexture(gl::TEXTURE_2D, texture0);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        let img = image::open(&Path::new("textures/container.jpg")).expect("Failed to load texture");
        let data = img.raw_pixels();
        gl::TexImage2D(gl::TEXTURE_2D,
                        0,
                        gl::RGB as i32,
                        img.width() as i32,
                        img.height() as i32,
                        0,
                        gl::RGB,
                        gl::UNSIGNED_BYTE,
                        &data[0] as *const u8 as *const c_void
                    );
        gl::GenerateMipmap(gl::TEXTURE_2D);

        let mut texture1 = 0;
        gl::GenTextures(1, &mut texture1);
        gl::BindTexture(gl::TEXTURE_2D, texture1);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        let img = image::open(&Path::new("textures/awesomeface.png")).expect("Failed to load texture");
        let img = img.flipv();
        let data = img.raw_pixels();
        gl::TexImage2D(gl::TEXTURE_2D,
                        0,
                        gl::RGB as i32,
                        img.width() as i32,
                        img.height() as i32,
                        0,
                        gl::RGBA,
                        gl::UNSIGNED_BYTE,
                        &data[0] as *const u8 as *const c_void
                    );
        gl::GenerateMipmap(gl::TEXTURE_2D);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        gl::BindVertexArray(0);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);

        (Shader::new("shaders/shader.vert", "shaders/shader.frag"), VAO, texture0, texture1)

    };


    while !window.should_close() {

        process_events(&events);
        process_input(&mut window, 0.01, &mut cam_pos, &mut cam_front);

        shaderProgram.useProgram();
        shaderProgram.setInt("tex0", 0);
        shaderProgram.setInt("tex1", 1);


        unsafe {
            gl::ClearColor(0.0, 0.5, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture0);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, texture1);

            let gltime = glfw.get_time() as f32;
            let model = Matrix4::<f32>::identity();
            let view: Matrix4<f32> = Matrix4::look_at(Point3::new(cam_pos.x, cam_pos.y, cam_pos.z), Point3::new(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0));
            let proj: Matrix4<f32> = perspective(Deg(45.0), 800.0/600.0 as f32, 0.1, 100.0);

            shaderProgram.useProgram();

            shaderProgram.setFloat("u_mix_param", gltime.sin().abs());
            shaderProgram.setMat4("u_model", model);
            shaderProgram.setMat4("u_view", view);
            shaderProgram.setMat4("u_projection", proj);

            gl::BindVertexArray(VAO);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }

        window.swap_buffers();
        glfw.poll_events();
    }
}

fn process_events(events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                unsafe {gl::Viewport(0, 0, width, height);}
            },
            _ => {}
        }
    }
}

fn process_input(window: &mut glfw::Window, delta_time: f32, camera_pos: &mut Point3<f32>, camera_front: &mut Vector3<f32>){
    if window.get_key(Key::Escape) == Action::Press {
        window.set_should_close(true);
    }

    let camera_speed: f32 = 1.0*delta_time;
    if window.get_key(Key::W) == Action::Press {
        *camera_pos += camera_speed * *camera_front;
    }
    if window.get_key(Key::S) == Action::Press {
        *camera_pos -= camera_speed * *camera_front;
    }
    if window.get_key(Key::A) == Action::Press {
        *camera_pos -= camera_speed * camera_front.cross(vec3(0.0, 1.0, 0.0)).normalize();
    }
    if window.get_key(Key::D) == Action::Press {
        *camera_pos += camera_speed * camera_front.cross(vec3(0.0, 1.0, 0.0)).normalize();
    }
}