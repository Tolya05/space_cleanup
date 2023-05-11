use macroquad::{prelude::{Rect, Texture2D, screen_width}, rand::gen_range};

pub struct SpaceObject {
    pub position: Rect,
    pub points: i32,
    pub texture: Texture2D,
    pub rotate: f32,
    pub health: i32
}

impl SpaceObject {
    pub fn new(texture: Texture2D) -> Self {
        Self{ position: Rect { x: gen_range(0.0, screen_width() - 64.0), y: gen_range(0.0, 50.0), w: 64.0, h: 64.0 }, points: gen_range(1, 5), texture: texture, rotate: gen_range(0.0, 360.0), health: -1 }
    }
}