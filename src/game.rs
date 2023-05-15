use macroquad::{prelude::Texture2D, audio::Sound};

pub trait Game {
    fn new(
        player_texture: Texture2D, 
        space_junk_textures: Vec<Texture2D>,
        game_music: Vec<Sound>,
        game_sounds: Vec<Sound>
    ) -> Self;
    fn update(&mut self);
    fn draw(&mut self);
    
    fn run(&mut self) {
        self.update();
        self.draw();
    }
}

pub fn exit_game() -> ! {
    std::process::exit(0);
}