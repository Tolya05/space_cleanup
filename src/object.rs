use macroquad::prelude::{Texture2D, Rect};

pub trait Object {
    fn new(texture: Texture2D) -> Self;
    fn get_x(&self) -> f32;
    fn get_y(&self) -> f32;
    fn get_width(&self) -> f32;
    fn get_height(&self) -> f32;
    fn get_rect(&self) -> Rect;
    fn move_x(&mut self, speed: f32);
    fn move_y(&mut self, speed: f32);
}