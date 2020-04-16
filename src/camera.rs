use cgmath::prelude::*;
use cgmath::{Point3, Vector3, vec3, Matrix4};

const CAM_FRONT: Vector3<f32> = vec3(0.0, 0.0, -1.0);
const CAM_POS: Point3<f32> = Point3::new(0.0, 0.0, 0.0);
const YAW: f32 = 270.0;
const PITCH: f32 = 0.0;

pub enum Direction {
    Forward,
    Backward, 
    Left,
    Right,
}

pub struct Camera{
    pub pos: Point3<f32>,
    pub front: Vector3<f32>,
    
    up: Vector3<f32>,
    yaw: f32,
    pitch: f32,
}

impl Camera{
    pub fn new() -> Camera{
        Camera{
            pos: CAM_POS,
            front: CAM_FRONT,
            up: vec3(0.0, 1.0, 0.0),
            yaw: YAW,
            pitch: PITCH
        }
    }

    pub fn get_view(&self) -> Matrix4<f32>{
        return Matrix4::look_at(self.pos, self.pos+self.front, self.up);
    }

    pub fn translate(&mut self, dir: Direction, delta_time: &f32){
        let camera_speed: f32 = 5.0*delta_time;

        match dir{
            Direction::Forward => self.pos += camera_speed * self.front,
            Direction::Backward => self.pos -= camera_speed * self.front,
            Direction::Left => self.pos -= camera_speed * self.front.cross(self.up).normalize(),
            Direction::Right => self.pos += camera_speed * self.front.cross(self.up).normalize(),
        }
    }

    pub fn turn(&mut self, yaw: f32, pitch: f32){
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