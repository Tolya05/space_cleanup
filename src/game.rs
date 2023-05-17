use macroquad::{prelude::{Texture2D, load_texture}, audio::{Sound, load_sound}};

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

pub async fn init_texture(path:  &str) -> Texture2D {
    let texture_result = load_texture(path).await;
    let texture: Texture2D;
    match texture_result {
        Ok(image) => texture = image.clone(),
        Err(error) => {
            println!("{error}");
            exit_game();
        },
    }
    texture
}

pub async fn init_sound(path:  &str) -> Sound {
    let sound_result = load_sound(path).await;
    let sound: Sound;
    match sound_result {
        Ok(sound_loaded) => sound = sound_loaded.clone(),
        Err(error) => {
            println!("{error}");
            exit_game();
        },
    }
    sound
}
