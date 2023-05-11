use macroquad::prelude::{Rect, Texture2D};

pub struct SpaceObject {
    pub position: Rect,
    pub points: i32,
    pub texture: Texture2D,
    pub rotate: f32,
    pub health: i32
}