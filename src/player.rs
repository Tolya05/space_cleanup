use macroquad::prelude::{Rect, Texture2D, screen_height, screen_width};
use crate::object::Object;

pub struct Player {
    position: Rect,
    pub health: i32,
    pub points: i32,
    pub coins: i32,
    pub texture: Texture2D,
}

impl Object for Player {
    fn new(texure: Texture2D) -> Self {
        Self {
            position: Rect {
                x: screen_width() / 2.0,
                y: screen_height() / 10.0 * 7.5,
                w: 75.0,
                h: 125.0
            },
            health: 5,
            points: 0,
            coins: 0,
            texture: texure
        }
    }
    fn get_x(&self) -> f32 {
        self.position.x.clone()
    }
    fn get_y(&self) -> f32 {
        self.position.y.clone()
    }
    fn get_width(&self) -> f32 {
        self.position.w.clone()
    }
    fn get_height(&self) -> f32 {
        self.position.h.clone()
    }
    fn get_rect(&self) -> Rect {
        self.position.clone()
    }
    fn move_x(&mut self, speed: f32) {
        self.position.x += speed;
    }
    fn move_y(&mut self, speed: f32) {
        self.position.y += speed;
    }
}