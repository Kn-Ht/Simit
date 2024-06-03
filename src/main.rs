use balls::Balls;
use galaxy::Galaxy;
use macroquad::{prelude::*, ui::{hash, root_ui, widgets}};
use scenes::*;
mod scenes;

fn draw_ui(scene_id: &mut usize, scenes: &mut Scenes) {
    widgets::Window::new(hash!(), Vec2::new(10.0, 10.0), Vec2::new(150.0, 500.0))
        .titlebar(true)
        .movable(true)
        .label("Simit settings")
        .ui(&mut *root_ui(), |ui| {
            ui.label(None, "Scene controls:");
            ui.label(None, &format!("Scene: {}", scenes[*scene_id].name()));
            ui.group(hash!(), Vec2::new(150.0, 20.0), |ui| {
                if ui.button(Vec2::ZERO, "< Prev   ") {
                    *scene_id = if *scene_id == 0 {SCENE_COUNT-1} else {*scene_id - 1}
                }
                if ui.button(Vec2::new(75.0, 0.0), "   Next >") {
                    *scene_id = if *scene_id == SCENE_COUNT-1 {0} else {*scene_id + 1}
                }
            });
        });
}

#[macroquad::main("Simit")]
async fn main() {
    let mut scene_id = 0;
    let mut scenes: Scenes = [
        Box::new(Galaxy::new()),
        Box::new(Balls::new())
    ];

    loop {
        println!("{scene_id}");

        let scene = &mut scenes[scene_id];
        scene.update();
        scene.draw();

        draw_ui(&mut scene_id, &mut scenes);

        next_frame().await;
    }
}
