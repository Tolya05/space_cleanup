use macroquad::prelude::{Rect, Texture2D};

pub struct Player {
    pub position: Rect,
    pub health: i32,
    pub points: i32,
    pub coins: i32,
    pub texture: Texture2D,
}