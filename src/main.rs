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


    let (shaderProgram, lampShader, outlineShader) = unsafe {

        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::STENCIL_TEST);
        (Shader::new("shaders/shader.vert", "shaders/shader.frag"), Shader::new("shaders/lamp.vert", "shaders/lamp.frag"), Shader::new("shaders/shader.vert", "shaders/outlineShader.frag"))

    };

    let light_positions: [Vector3<f32>; 6] = [
        vec3( 0.7,  0.2,  2.0),
        vec3( 2.3, -3.3, -4.0),
        vec3(-4.0,  2.0, -12.0),
        vec3( 0.0,  0.0, -3.0),
        vec3( 5.0,  0.0, 0.0),
        vec3( 0.0,  0.0, -6.0)
    ];

    let model = Model::new("models/corona.obj");
    let cube_model = Model::new("models/cube.obj");

    while !window.should_close() {

        let current_time = glfw.get_time() as f32;
        delta_time = current_time - lastFrame;
        lastFrame = current_time;

        process_events(&events, &mut first_mouse, &mut lastX, &mut lastY, &mut camera);
        process_input(&mut window, &delta_time, &mut camera);

        shaderProgram.useProgram();

        unsafe {
            gl::ClearColor(0.0, 0.5, 0.5, 1.0);
            gl::Enable(gl::DEPTH_TEST);
            gl::StencilOp(gl::KEEP, gl::KEEP, gl::KEEP);
            gl::StencilMask(0xFF);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);

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

            model.draw(&shaderProgram);

            gl::StencilOp(gl::KEEP, gl::KEEP, gl::REPLACE);
            gl::StencilFunc(gl::ALWAYS, 1, 0xFF);
            gl::StencilMask(0xFF);

            let view: Matrix4<f32> = camera.get_view();
            let proj: Matrix4<f32> = perspective(Deg(45.0), 800.0/600.0 as f32, 0.1, 100.0);
            
            lampShader.useProgram();
            lampShader.setMat4("u_view", view);
            lampShader.setMat4("u_projection", proj);

            for position in light_positions.iter(){
                let model = Matrix4::<f32>::from_translation(*position)*Matrix4::<f32>::from_scale(0.2);
                lampShader.setMat4("u_model", model);
                
                cube_model.draw(&lampShader);
            }

            gl::StencilFunc(gl::NOTEQUAL, 1, 0xFF);
            gl::StencilMask(0x00);
            gl::Disable(gl::DEPTH_TEST);

            outlineShader.useProgram();
            outlineShader.setMat4("u_view", view);
            outlineShader.setMat4("u_projection", proj);

            for position in light_positions.iter(){
                let model = Matrix4::<f32>::from_translation(*position)*Matrix4::<f32>::from_scale(0.25);
                outlineShader.setMat4("u_model", model);
                
                cube_model.draw(&outlineShader);
            }


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