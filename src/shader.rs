use std::ffi::CString;
use std::str;
use std::fs::File;
use std::io::Read;
use std::ptr;

use gl::types::*;

use cgmath::prelude::*;
use cgmath::Matrix4;

pub struct Shader{
    id: u32,
}

impl Shader{
    pub fn newGeometry(vertexPath: &str, fragmentPath: &str, geometryPath: &str) -> Shader{
        let mut res = Shader{ id:0 };
        
        let mut vertexShaderSourceFile = File::open(vertexPath).unwrap();
        let mut fragmentShaderSourceFile = File::open(fragmentPath).unwrap();
        let mut geometryShaderSourceFile = File::open(geometryPath).unwrap();

        let mut vertexShaderSource = String::new();
        let mut fragmentShaderSource = String::new();
        let mut geometryShaderSource = String::new();
        
        vertexShaderSourceFile.read_to_string(&mut vertexShaderSource).unwrap();
        fragmentShaderSourceFile.read_to_string(&mut fragmentShaderSource).unwrap();
        geometryShaderSourceFile.read_to_string(&mut geometryShaderSource).unwrap();

        let vertexShaderSource = CString::new(vertexShaderSource.as_bytes()).unwrap();
        let fragmentShaderSource = CString::new(fragmentShaderSource.as_bytes()).unwrap();
        let geometryShaderSource = CString::new(geometryShaderSource.as_bytes()).unwrap();

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

            let geometryShader = gl::CreateShader(gl::GEOMETRY_SHADER);
            gl::ShaderSource(geometryShader, 1, &geometryShaderSource.as_ptr(), ptr::null());
            gl::CompileShader(geometryShader);

            gl::GetShaderiv(geometryShader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(geometryShader, 512, ptr::null_mut(), infoLog.as_mut_ptr() as *mut GLchar);
                println!("{}", str::from_utf8_unchecked(&infoLog));
            }

            let shaderProgram = gl::CreateProgram();
            gl::AttachShader(shaderProgram, vertexShader);
            gl::AttachShader(shaderProgram, fragmentShader);
            gl::AttachShader(shaderProgram, geometryShader);
            gl::LinkProgram(shaderProgram);

            gl::DeleteShader(vertexShader);
            gl::DeleteShader(fragmentShader);
            gl::DeleteShader(geometryShader);

            res.id = shaderProgram;
        }

        res
    }

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

    pub fn setUniform3f(&self, name: &str, vector: (f32, f32, f32)){
        let name = CString::new(name.as_bytes()).unwrap();
        unsafe{
            gl::Uniform3f(gl::GetUniformLocation(self.id, name.as_ptr()), vector.0, vector.1, vector.2);
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

    pub fn bindUniformBlock(&self, name: &str, value: u32){
        let name = CString::new(name.as_bytes()).unwrap();
        unsafe{
            gl::UniformBlockBinding(self.id, gl::GetUniformBlockIndex(self.id, name.as_ptr()), value);
        }
    }
}
