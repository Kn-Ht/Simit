pub mod galaxy;
pub mod balls;

pub trait Scene {
    fn name(&self) -> &'static str;
    fn update(&mut self);
    fn draw(&self);
}

pub const SCENE_COUNT: usize = 2;
pub type Scenes = [Box<dyn Scene>; SCENE_COUNT];