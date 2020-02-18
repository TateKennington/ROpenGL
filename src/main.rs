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

const CAM_FRONT: Vector3<f32> = vec3(0.0, 0.0, -1.0);
const CAM_POS: Point3<f32> = Point3::new(0.0, 0.0, 0.0);
const YAW: f32 = 0.0;
const PITCH: f32 = 0.0;

pub enum Direction {
    Forward,
    Backward, 
    Left,
    Right,
}

struct Camera{
    pos: Point3<f32>,
    front: Vector3<f32>,
    up: Vector3<f32>,
    yaw: f32,
    pitch: f32,
}

impl Camera{
    fn new() -> Camera{
        Camera{
            pos: CAM_POS,
            front: CAM_FRONT,
            up: vec3(0.0, 1.0, 0.0),
            yaw: YAW,
            pitch: PITCH
        }
    }

    fn get_view(&self) -> Matrix4<f32>{
        return Matrix4::look_at(self.pos, self.pos+self.front, self.up);
    }

    fn translate(&mut self, dir: Direction, delta_time: &f32){
        let camera_speed: f32 = 5.0*delta_time;

        match dir{
            Direction::Forward => self.pos += camera_speed * self.front,
            Direction::Backward => self.pos -= camera_speed * self.front,
            Direction::Left => self.pos -= camera_speed * self.front.cross(self.up).normalize(),
            Direction::Right => self.pos += camera_speed * self.front.cross(self.up).normalize(),
        }
    }

    fn turn(&mut self, yaw: f32, pitch: f32){
        self.yaw += yaw;
        self.pitch += pitch;

        if self.pitch > 89.0{
            self.pitch = 89.0;
        }
        if self.pitch < -89.0{
            self.pitch = -89.0;
        }

        self.front = Vector3{
            x: self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            y: self.pitch.to_radians().sin(),
            z: self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        };
    }
}

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
    window.set_cursor_pos_polling(true);

    window.set_cursor_mode(glfw::CursorMode::Disabled);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let mut camera = Camera::new();
    let mut light_pos: Vector3<f32> = vec3(1.2, 1.0, 2.0);
    let mut first_mouse = true;
    let mut lastX: f32 = 0.0;
    let mut lastY: f32 = 0.0;

    let mut lastFrame: f32 = 0.0;
    let mut delta_time: f32;


    let (shaderProgram, VAO, lampShader, lampVAO) = unsafe {

        gl::Enable(gl::DEPTH_TEST);

        let vertices: [f32; 108] = [
            -0.5, -0.5, -0.5,
             0.5, -0.5, -0.5,
             0.5,  0.5, -0.5,
             0.5,  0.5, -0.5,
            -0.5,  0.5, -0.5,
            -0.5, -0.5, -0.5,

            -0.5, -0.5,  0.5,
             0.5, -0.5,  0.5,
             0.5,  0.5,  0.5,
             0.5,  0.5,  0.5,
            -0.5,  0.5,  0.5,
            -0.5, -0.5,  0.5,

            -0.5,  0.5,  0.5,
            -0.5,  0.5, -0.5,
            -0.5, -0.5, -0.5,
            -0.5, -0.5, -0.5,
            -0.5, -0.5,  0.5,
            -0.5,  0.5,  0.5,

             0.5,  0.5,  0.5,
             0.5,  0.5, -0.5,
             0.5, -0.5, -0.5,
             0.5, -0.5, -0.5,
             0.5, -0.5,  0.5,
             0.5,  0.5,  0.5,

            -0.5, -0.5, -0.5,
             0.5, -0.5, -0.5,
             0.5, -0.5,  0.5,
             0.5, -0.5,  0.5,
            -0.5, -0.5,  0.5,
            -0.5, -0.5, -0.5,

            -0.5,  0.5, -0.5,
             0.5,  0.5, -0.5,
             0.5,  0.5,  0.5,
             0.5,  0.5,  0.5,
            -0.5,  0.5,  0.5,
            -0.5,  0.5, -0.5,
        ];

        let (mut VBO, mut VAO, mut lampVAO) = (0, 0, 0);
        gl::GenVertexArrays(1, &mut VAO);
        gl::GenBuffers(1, &mut VBO);

        gl::BindVertexArray(VAO);
        
        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
        gl::BufferData(gl::ARRAY_BUFFER,
                        (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                        &vertices[0] as *const f32 as *const c_void,
                        gl::STATIC_DRAW);

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::GenVertexArrays(1, &mut lampVAO);
        
        gl::BindVertexArray(lampVAO);

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        gl::BindVertexArray(0);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);

        (Shader::new("shaders/shader.vert", "shaders/shader.frag"), VAO, Shader::new("shaders/lamp.vert", "shaders/lamp.frag"), lampVAO)

    };


    while !window.should_close() {

        let current_time = glfw.get_time() as f32;
        delta_time = current_time - lastFrame;
        lastFrame = current_time;

        process_events(&events, &mut first_mouse, &mut lastX, &mut lastY, &mut camera);
        process_input(&mut window, &delta_time, &mut camera);

        shaderProgram.useProgram();
        shaderProgram.setInt("tex0", 0);
        shaderProgram.setInt("tex1", 1);


        unsafe {
            gl::ClearColor(0.0, 0.5, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            let model = Matrix4::<f32>::identity();
            let view: Matrix4<f32> = camera.get_view();
            let proj: Matrix4<f32> = perspective(Deg(45.0), 800.0/600.0 as f32, 0.1, 100.0);

            shaderProgram.useProgram();

            shaderProgram.setMat4("u_model", model);
            shaderProgram.setMat4("u_view", view);
            shaderProgram.setMat4("u_projection", proj);
            shaderProgram.setUniform4f("object_color", (0.0, 0.5, 0.3, 1.0));
            shaderProgram.setUniform4f("light_color", (1.0, 1.0, 1.0, 1.0));

            gl::BindVertexArray(VAO);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            let model = Matrix4::<f32>::from_translation(light_pos)*Matrix4::<f32>::from_scale(0.2);
            let view: Matrix4<f32> = camera.get_view();
            let proj: Matrix4<f32> = perspective(Deg(45.0), 800.0/600.0 as f32, 0.1, 100.0);

            lampShader.useProgram();

            lampShader.setMat4("u_model", model);
            lampShader.setMat4("u_view", view);
            lampShader.setMat4("u_projection", proj);
            
            gl::BindVertexArray(lampVAO);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }

        window.swap_buffers();
        glfw.poll_events();
    }
}

fn process_events(events: &Receiver<(f64, glfw::WindowEvent)>, first_mouse: &mut bool, lastX: &mut f32, lastY: &mut f32, camera: &mut Camera) {

    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                unsafe {gl::Viewport(0, 0, width, height);}
            },
            glfw::WindowEvent::CursorPos(xpos, ypos) => {
                let (xpos, ypos) = (xpos as f32, ypos as f32);
                if *first_mouse{
                    *lastX = xpos;
                    *lastY = ypos;
                    *first_mouse = false;
                }

                let mut xoff = xpos - *lastX;
                let mut yoff = *lastY - ypos;
                *lastX = xpos;
                *lastY = ypos;

                xoff *= 0.1;
                yoff *= 0.1;

                camera.turn(xoff, yoff);
            },
            _ => {}
        }
    }
}

fn process_input(window: &mut glfw::Window, delta_time: &f32, camera: &mut Camera){
    if window.get_key(Key::Escape) == Action::Press {
        window.set_should_close(true);
    }

    if window.get_key(Key::W) == Action::Press {
        camera.translate(Direction::Forward, delta_time);
    }
    if window.get_key(Key::S) == Action::Press {
        camera.translate(Direction::Backward, delta_time);
    }
    if window.get_key(Key::A) == Action::Press {
        camera.translate(Direction::Left, delta_time);
    }
    if window.get_key(Key::D) == Action::Press {
        camera.translate(Direction::Right, delta_time);
    }
}