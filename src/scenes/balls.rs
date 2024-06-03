use macroquad::prelude::*;

use super::Scene;

pub struct Balls {
    camera: Camera3D
}

impl Scene for Balls {
    fn draw(&self) {
        clear_background(RED);
    }
    fn update(&mut self) {

    }
    fn name(&self) -> &'static str {
        "Balls"
    }
}

impl Balls {
    pub fn new() -> Self {
        Self {
            camera: Camera3D {
                position: vec3(10.0, 8.0, 0.0),
                up: vec3(0., 1., 0.),
                target: vec3(0., 0., 0.),
                fovy: 45.0,
                ..Default::default()
            }
        }
    }
}