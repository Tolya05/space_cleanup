use macroquad::{prelude::{Rect, Texture2D, screen_width}, rand::gen_range, audio::{PlaySoundParams, Sound, play_sound}};

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
    pub fn reset(&mut self) {
        self.position.x = gen_range(0.0, screen_width());
        self.position.y = gen_range(0.0, 50.0);
        self.rotate = gen_range(0.0, 360.0);
    }
    pub fn hit(&mut self, sound: Sound, volume: f32) {
        play_sound(sound, PlaySoundParams { looped: false, volume: volume});
        self.position.x = gen_range(0.0, screen_width() - self.position.w);
        self.position.y = gen_range(0.0, 50.0);
        self.rotate = gen_range(0.0, 360.0);
    }
    pub fn get_points(&self) -> i32 {
        self.points.clone()
    }
    pub fn get_health(&self) -> i32 {
        self.health.clone()
    }
    pub fn move_y(&mut self, speed: f32) {
        self.position.y += speed;
    }
}