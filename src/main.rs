extern crate glfw;
extern crate gl;

mod macros;
mod mesh;
mod model;
use model::Model;

mod camera;
use camera::Camera;
use camera::Direction;

mod shader;
use shader::Shader;

use glfw::{Context, Key, Action};
use gl::types::*;

use cgmath::{Matrix4, vec3, Deg, perspective, Vector3};
use cgmath::prelude::*;

use std::sync::mpsc::Receiver;
use std::ptr;
use std::mem;
use std::os::raw::c_void;

use std::path::Path;
use image;
use image::DynamicImage::*;
use image::GenericImageView;

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
    let mut first_mouse = true;
    let mut lastX: f32 = 0.0;
    let mut lastY: f32 = 0.0;

    let mut lastFrame: f32 = 0.0;
    let mut delta_time: f32;


    let ( postproShader, shaderProgram, lampShader, outlineShader, transparentShader, skyboxShader, reflectionShader, quadVAO, fbo, color_buffer, skybox, cubeVAO, containerVAO) = unsafe {

        let mut fbo = 0;
        gl::GenFramebuffers(1, &mut fbo);
        gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);

        let mut tex = 0;
        gl::GenTextures(1, &mut tex);
        gl::BindTexture(gl::TEXTURE_2D, tex);
        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, 800, 600, 0, gl::RGB, gl::UNSIGNED_BYTE, ptr::null());
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, tex, 0);
        gl::BindTexture(gl::TEXTURE_2D, 0);

        let mut rbo = 0;
        gl::GenRenderbuffers(1, &mut rbo);
        gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);
        gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH24_STENCIL8, 800, 600);
        gl::BindRenderbuffer(gl::RENDERBUFFER, 0);

        gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_STENCIL_ATTACHMENT, gl::RENDERBUFFER, rbo);
        
        if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
            println!("ERROR::FRAMEBUFFER:: Framebuffer is not complete!");
        }
        println!("Made Framebuffer good");
        
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LEQUAL);
        gl::Enable(gl::STENCIL_TEST);
        //gl::Enable(gl::CULL_FACE);
        gl::Enable(gl::BLEND);
        gl::CullFace(gl::FRONT);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        
        let mut skybox = 0;
        gl::GenTextures(1, &mut skybox);
        gl::BindTexture(gl::TEXTURE_CUBE_MAP, skybox);

        let skybox_paths = vec!(
            "textures/right.jpg",
            "textures/left.jpg",
            "textures/top.jpg",
            "textures/bottom.jpg",
            "textures/back.jpg",
            "textures/front.jpg"
        );

        for (i, path) in skybox_paths.iter().enumerate(){

            let img = image::open(&Path::new(path)).expect("Failed");
            let format = match img{
                ImageLuma8(_) => gl::RED,
                ImageLumaA8(_) => gl::RG,
                ImageRgb8(_) => gl::RGB,
                ImageRgba8(_) => gl::RGBA,
                _ => panic!("Unsupported image format")
            };

            let data = img.raw_pixels();

            gl::TexImage2D(gl::TEXTURE_CUBE_MAP_POSITIVE_X + i as u32, 0, format as i32, img.width() as i32, img.height() as i32,
                            0, format, gl::UNSIGNED_BYTE, &data[0] as *const u8 as *const c_void);
        }

        gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        gl::BindTexture(gl::TEXTURE_CUBE_MAP, 0);

        let mut cubeVAO = 0;
        let mut cubeVBO = 0;

        gl::GenVertexArrays(1, &mut cubeVAO);
        gl::GenBuffers(1, &mut cubeVBO);

        let vertices: [f32; 108] = [
            // positions          
            -1.0,  1.0, -1.0,
            -1.0, -1.0, -1.0,
             1.0, -1.0, -1.0,
             1.0, -1.0, -1.0,
             1.0,  1.0, -1.0,
            -1.0,  1.0, -1.0,
        
            -1.0, -1.0,  1.0,
            -1.0, -1.0, -1.0,
            -1.0,  1.0, -1.0,
            -1.0,  1.0, -1.0,
            -1.0,  1.0,  1.0,
            -1.0, -1.0,  1.0,
        
             1.0, -1.0, -1.0,
             1.0, -1.0,  1.0,
             1.0,  1.0,  1.0,
             1.0,  1.0,  1.0,
             1.0,  1.0, -1.0,
             1.0, -1.0, -1.0,
        
            -1.0, -1.0,  1.0,
            -1.0,  1.0,  1.0,
             1.0,  1.0,  1.0,
             1.0,  1.0,  1.0,
             1.0, -1.0,  1.0,
            -1.0, -1.0,  1.0,
        
            -1.0,  1.0, -1.0,
             1.0,  1.0, -1.0,
             1.0,  1.0,  1.0,
             1.0,  1.0,  1.0,
            -1.0,  1.0,  1.0,
            -1.0,  1.0, -1.0,
        
            -1.0, -1.0, -1.0,
            -1.0, -1.0,  1.0,
             1.0, -1.0, -1.0,
             1.0, -1.0, -1.0,
            -1.0, -1.0,  1.0,
             1.0, -1.0,  1.0
        ];

        gl::BindVertexArray(cubeVAO);

        gl::BindBuffer(gl::ARRAY_BUFFER, cubeVBO);
        gl::BufferData(gl::ARRAY_BUFFER, (mem::size_of::<GLfloat>() * vertices.len()) as GLsizeiptr, &vertices[0] as *const f32 as *const c_void, gl::STATIC_DRAW);
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, (3 * mem::size_of::<GLfloat>()) as GLsizei, ptr::null());

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);

        let mut quadVAO = 0;
        let mut quadVBO = 0;

        gl::GenVertexArrays(1, &mut quadVAO);
        gl::GenBuffers(1, &mut quadVBO);

        let vertices: [f32; 30] = [
            -1.0, 1.0, 0.0, 0.0, 1.0,
            1.0, 1.0, 0.0, 1.0, 1.0,
            1.0, -1.0, 0.0, 1.0, 0.0,
            1.0, -1.0, 0.0, 1.0, 0.0,
            -1.0, -1.0, 0.0, 0.0, 0.0,
            -1.0, 1.0, 0.0, 0.0, 1.0,
        ];

        gl::BindVertexArray(quadVAO);

        gl::BindBuffer(gl::ARRAY_BUFFER, quadVBO);
        gl::BufferData(gl::ARRAY_BUFFER, (mem::size_of::<GLfloat>() * vertices.len()) as GLsizeiptr, &vertices[0] as *const f32 as *const c_void, gl::STATIC_DRAW);
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, (5 * mem::size_of::<GLfloat>()) as GLsizei, ptr::null());
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, (5 * mem::size_of::<GLfloat>()) as GLsizei, (3 * mem::size_of::<GLfloat>()) as *const c_void);

        gl::BindVertexArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        let mut containerVAO = 0;
        let mut containerVBO = 0;

        let vertices: [f32;216] = [
            -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,
            0.5, -0.5, -0.5,  0.0,  0.0, -1.0, 
            0.5,  0.5, -0.5,  0.0,  0.0, -1.0, 
            0.5,  0.5, -0.5,  0.0,  0.0, -1.0, 
            -0.5,  0.5, -0.5,  0.0,  0.0, -1.0, 
            -0.5, -0.5, -0.5,  0.0,  0.0, -1.0, 

            -0.5, -0.5,  0.5,  0.0,  0.0, 1.0,
            0.5, -0.5,  0.5,  0.0,  0.0, 1.0,
            0.5,  0.5,  0.5,  0.0,  0.0, 1.0,
            0.5,  0.5,  0.5,  0.0,  0.0, 1.0,
            -0.5,  0.5,  0.5,  0.0,  0.0, 1.0,
            -0.5, -0.5,  0.5,  0.0,  0.0, 1.0,

            -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,
            -0.5,  0.5, -0.5, -1.0,  0.0,  0.0,
            -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,
            -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,
            -0.5, -0.5,  0.5, -1.0,  0.0,  0.0,
            -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,

            0.5,  0.5,  0.5,  1.0,  0.0,  0.0,
            0.5,  0.5, -0.5,  1.0,  0.0,  0.0,
            0.5, -0.5, -0.5,  1.0,  0.0,  0.0,
            0.5, -0.5, -0.5,  1.0,  0.0,  0.0,
            0.5, -0.5,  0.5,  1.0,  0.0,  0.0,
            0.5,  0.5,  0.5,  1.0,  0.0,  0.0,

            -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,
            0.5, -0.5, -0.5,  0.0, -1.0,  0.0,
            0.5, -0.5,  0.5,  0.0, -1.0,  0.0,
            0.5, -0.5,  0.5,  0.0, -1.0,  0.0,
            -0.5, -0.5,  0.5,  0.0, -1.0,  0.0,
            -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,

            -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,
            0.5,  0.5, -0.5,  0.0,  1.0,  0.0,
            0.5,  0.5,  0.5,  0.0,  1.0,  0.0,
            0.5,  0.5,  0.5,  0.0,  1.0,  0.0,
            -0.5,  0.5,  0.5,  0.0,  1.0,  0.0,
            -0.5,  0.5, -0.5,  0.0,  1.0,  0.0
        ];

        gl::GenVertexArrays(1, &mut containerVAO);
        gl::GenBuffers(1, &mut containerVBO);

        gl::BindVertexArray(containerVAO);
        gl::BindBuffer(gl::ARRAY_BUFFER, containerVBO);
        gl::BufferData(gl::ARRAY_BUFFER, (mem::size_of::<GLfloat>() * vertices.len()) as GLsizeiptr, &vertices[0] as *const f32 as *const c_void, gl::STATIC_DRAW);
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, (6 * mem::size_of::<GLfloat>()) as GLsizei, ptr::null());
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, (6 * mem::size_of::<GLfloat>()) as GLsizei, (3 * mem::size_of::<GLfloat>()) as *const c_void);

        (
            Shader::new("shaders/postpro.vert", "shaders/postpro.frag"),
            Shader::new("shaders/shader.vert","shaders/shader.frag"),
            Shader::new("shaders/lamp.vert","shaders/lamp.frag"),
            Shader::new("shaders/shader.vert","shaders/outlineShader.frag"),
            Shader::new("shaders/shader.vert","shaders/transparentShader.frag"),
            Shader::new("shaders/skybox.vert", "shaders/skybox.frag"),
            Shader::new("shaders/shader.vert", "shaders/reflection.frag"),
            quadVAO,
            fbo,
            tex,
            skybox,
            cubeVAO,
            containerVAO
        )

    };

    let light_positions: [Vector3<f32>; 6] = [
        vec3( 0.7,  0.2,  2.0),
        vec3( 2.3, -3.3, -4.0),
        vec3(-4.0,  2.0, -12.0),
        vec3( 0.0,  0.0, -3.0),
        vec3( 5.0,  0.0, 0.0),
        vec3( 0.0,  0.0, -6.0)
    ];

    let mut windows_positions = Vec::<Vector3<f32>>::new();

    for i in 1..5 {
        windows_positions.push(Vector3::<f32>::unit_z() * i as f32);
    }

    let model = Model::new("models/corona.obj");
    let cube_model = Model::new("models/cube.obj");

    while !window.should_close() {

        let current_time = glfw.get_time() as f32;
        delta_time = current_time - lastFrame;
        lastFrame = current_time;

        process_events(&events, &mut first_mouse, &mut lastX, &mut lastY, &mut camera);
        process_input(&mut window, &delta_time, &mut camera);

        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            gl::ClearColor(0.0, 0.5, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
            gl::Enable(gl::DEPTH_TEST);

            let model_mat: Matrix4<f32> = Matrix4::identity();
            let view: Matrix4<f32> = camera.get_view();
            let proj: Matrix4<f32> = perspective(Deg(45.0), 800.0/600.0 as f32, 0.1, 100.0);

            shaderProgram.useProgram();
            
            shaderProgram.setUniform3f("dir_light.direction", (-0.2, -1.0, -0.3));
            shaderProgram.setUniform3f("dir_light.ambient", (0.2, 0.2, 0.2));
            shaderProgram.setUniform3f("dir_light.diffuse", (0.2, 0.2, 0.2));
            shaderProgram.setUniform3f("dir_light.specular", (0.2, 0.2, 0.2));

            shaderProgram.setUniform3f("spot_light.pos", (camera.pos.x, camera.pos.y, camera.pos.z));
            shaderProgram.setUniform3f("spot_light.direction", (camera.front.x, camera.front.y, camera.front.z));
            shaderProgram.setUniform3f("spot_light.ambient", (0.2, 0.2, 0.2));
            shaderProgram.setUniform3f("spot_light.diffuse", (0.5, 0.5, 0.5));
            shaderProgram.setUniform3f("spot_light.specular", (1.0, 1.0, 1.0));
            shaderProgram.setFloat("spot_light.cutoff", (0.2 as f32).cos());
            shaderProgram.setFloat("spot_light.outerCutoff", (0.3 as f32).cos());

            shaderProgram.setMat4("u_model", model_mat);
            shaderProgram.setMat4("u_view", view);
            shaderProgram.setMat4("u_projection", proj);

            shaderProgram.setUniform3f("camera_pos", (camera.pos.x, camera.pos.y, camera.pos.z));

            shaderProgram.setInt("material.diffuse", 0);
            shaderProgram.setInt("material.specular", 1);
            shaderProgram.setFloat("material.shininess", 32.0);
            
            for (i, position) in light_positions.iter().enumerate(){

                shaderProgram.setUniform3f(&format!("point_lights[{}].pos", i), (position.x, position.y, position.z));

                shaderProgram.setUniform3f(&format!("point_lights[{}].ambient", i), (0.2, 0.2, 0.2));
                shaderProgram.setUniform3f(&format!("point_lights[{}].diffuse", i), (0.5, 0.5, 0.5));
                shaderProgram.setUniform3f(&format!("point_lights[{}].specular", i), (1.0, 1.0, 1.0));

                shaderProgram.setFloat(&format!("point_lights[{}].c", i), 1.0);
                shaderProgram.setFloat(&format!("point_lights[{}].l", i), 0.09);
                shaderProgram.setFloat(&format!("point_lights[{}].q", i), 0.05);
            }
            
            reflectionShader.useProgram();
            reflectionShader.setMat4("u_view", view);
            reflectionShader.setMat4("u_projection", proj);
            reflectionShader.setMat4("u_model", model_mat);
            reflectionShader.setUniform3f("camera_pos", (camera.pos.x, camera.pos.y, camera.pos.z));
            reflectionShader.setInt("skybox", 0);
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, skybox);
            model.draw(&reflectionShader);

            
            gl::StencilOp(gl::KEEP, gl::KEEP, gl::REPLACE);
            gl::StencilFunc(gl::ALWAYS, 1, 0xFF);
            gl::StencilMask(0xFF);
            
            lampShader.useProgram();
            lampShader.setMat4("u_view", view);
            lampShader.setMat4("u_projection", proj);

            for position in light_positions.iter(){
                let model = Matrix4::<f32>::from_translation(*position)*Matrix4::<f32>::from_scale(0.2);
                lampShader.setMat4("u_model", model);
                
                cube_model.draw(&lampShader);
            }

            gl::StencilOp(gl::KEEP, gl::KEEP, gl::KEEP);
            gl::StencilFunc(gl::ALWAYS, 1, 0xFF);
            gl::StencilMask(0xFF);
            transparentShader.useProgram();

            let model_mat: Matrix4<f32> = Matrix4::identity();

            transparentShader.setMat4("u_model", model_mat);
            transparentShader.setMat4("u_view", view);
            transparentShader.setMat4("u_projection", proj);

            windows_positions.sort_by(|a, b| {
                let pos = camera.pos.to_vec();
                (pos.distance2(*a)).partial_cmp(&pos.distance2(*b)).unwrap().reverse()
            });

            gl::BindVertexArray(quadVAO);
            for position in windows_positions.iter(){
                let model_mat = Matrix4::<f32>::from_translation(*position);
                transparentShader.setMat4("u_model", model_mat);
                gl::DrawArrays(gl::TRIANGLES, 0, 6);
            }
            gl::BindVertexArray(0);
         
            let mut view = view;

            skyboxShader.useProgram();

            view.w[0] = 0.0;
            view.w[1] = 0.0;
            view.w[2] = 0.0;
            skyboxShader.setMat4("u_view", view);
            skyboxShader.setMat4("u_proj", proj);

            gl::DepthFunc(gl::LEQUAL);
            gl::BindVertexArray(cubeVAO);
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, skybox);
            skyboxShader.setInt("skybox", 0);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
            gl::DepthFunc(gl::LESS);

            gl::StencilFunc(gl::NOTEQUAL, 1, 0xFF);
            gl::StencilMask(0x00);
            gl::Disable(gl::DEPTH_TEST);

            let view: Matrix4<f32> = camera.get_view();

            outlineShader.useProgram();
            outlineShader.setMat4("u_view", view);
            outlineShader.setMat4("u_projection", proj);

            for position in light_positions.iter(){
                let model = Matrix4::<f32>::from_translation(*position)*Matrix4::<f32>::from_scale(0.25);
                outlineShader.setMat4("u_model", model);
                
                cube_model.draw(&outlineShader);
            }

            gl::Enable(gl::DEPTH_TEST);
            gl::StencilOp(gl::KEEP, gl::KEEP, gl::KEEP);
            gl::StencilFunc(gl::ALWAYS, 1, 0xFF);
            gl::StencilMask(0xFF);
            /* gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            gl::ClearColor(0.0, 0.5, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
            gl::Disable(gl::DEPTH_TEST);

            postproShader.useProgram();
            gl::BindTexture(gl::TEXTURE_2D, color_buffer);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindVertexArray(quadVAO);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            gl::BindVertexArray(0); */

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