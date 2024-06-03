use macroquad::prelude::*;

use super::{Scene, SCENE_COUNT};

const BG_COLOR: Color = Color::new(0.2, 0.2, 0.2, 1.0);

pub struct Galaxy {
    camera: Camera3D,
}

impl Scene for Galaxy {
    fn name(&self) -> &'static str {
        "Galaxy"
    }
    fn update(&mut self) {
        self.rotate_camera(get_frame_time(), 0.1);
    }
    fn draw(&self) {
        clear_background(BG_COLOR);
        set_camera(&self.camera);
        
        draw_grid(10, 1.0, WHITE, WHITE);
    }
}

impl Galaxy {
    pub fn new() -> Self {
        Self {
            camera: Camera3D {
                position: vec3(10.0, 8.0, 0.0),
                up: vec3(0., 1., 0.),
                target: vec3(0., 0., 0.),
                fovy: 45.0,
                ..Default::default()
            },
        }
    }
    fn rotate_camera(&mut self, dt: f32, rot_speed: f32) {
        let camera = &mut self.camera;
        let speed = rot_speed * dt;
        let v_cos = speed.cos();
        let v_sin = speed.sin();
        camera.position.x = camera.position.x * v_cos - camera.position.z * v_sin;
        camera.position.z = camera.position.z * v_cos + camera.position.x * v_sin;
    }
}
