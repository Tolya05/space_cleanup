mod game;
mod object;
mod player;
mod space_objects;
mod timer;

use macroquad::{prelude::*, rand::*, audio::{Sound, PlaySoundParams, play_sound, stop_sound}};
use egui_macroquad::egui::{self, Pos2};
use player::Player;
use space_objects::SpaceObject;
use timer::Timer;
use game::{Game, exit_game, init_texture, init_sound};
use object::Object;
use macroui::button::Button;

#[derive(Clone, Copy)]
enum State {
    MainMenu,
    Credits,
    GameTutorial,
    Game,
    Options,
    Shop,
    GameOver,
}

fn calculate_speed(points: i32) -> f32 {
    let mut speed: f32 = (points as f32) * 2.0 + 250.0;
    if speed > 3500.0 {
        speed = 3500.0;
    }
    speed
}

struct GameStruct {
    player: Player,
    scraps: Vec<SpaceObject>,
    asteroids: Vec<SpaceObject>,
    debug: bool,
    paused: bool,
    state: State,
    previous_state: State,
    game_music: Vec<Sound>,
    game_sounds: Vec<Sound>,
    music_timer: Timer,
    music_volume: f32,
    sound_volume: f32
}

impl GameStruct {
    fn draw_pause(&mut self) {
        let window_frame = egui::containers::Frame{
            fill: egui::Color32::TRANSPARENT,
            ..Default::default()
        };
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("egui ❤ macroquad")
                .title_bar(false)
                .frame(window_frame)
                .default_pos(Pos2{ x: (screen_width() / 2.0 + 250.0) / 3.0, y: (screen_height() / 2.0 + 200.0) / 3.0 })
                .resizable(false)
                .show(egui_ctx, |ui| {
                    egui_ctx.set_pixels_per_point(3.0);
                    ui.label("Paused");
                    if ui.button("Continue").clicked() {
                        self.paused = false;
                    }
                    if ui.button("Go to Shop").clicked() {
                        self.state = State::Shop;
                    }
                    if ui.button("Credits").clicked() {
                        self.previous_state = State::Game;
                        self.state = State::Credits;
                    }
                    if ui.button("Options").clicked() {
                        self.previous_state = State::Game;
                        self.state = State::Options;
                    }
                    if ui.button("Quit").clicked() {
                        exit_game()
                    }
                });
        });
    }

    fn draw_main_menu(&mut self) {
        let window_frame = egui::containers::Frame{
            fill: egui::Color32::TRANSPARENT,
            ..Default::default()
        };
        clear_background(BLACK);
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("egui ❤ macroquad")
                .title_bar(false)
                .frame(window_frame)
                .default_pos(Pos2{ x: (screen_width() / 2.0 + 250.0) / 3.0, y: (screen_height() / 2.0 + 200.0) / 3.0 })
                .resizable(false)
                .show(egui_ctx, |ui| {
                    egui_ctx.set_pixels_per_point(3.0);
                    ui.label("Space Cleanup");
                    if ui.button("Play").clicked() {
                        self.state = State::GameTutorial;
                    }
                    if ui.button("Credits").clicked() {
                        self.previous_state = State::MainMenu;
                        self.state = State::Credits;
                    }
                    if ui.button("Options").clicked() {
                        self.previous_state = State::MainMenu;
                        self.state = State::Options;
                    }
                    if ui.button("Quit").clicked() {
                        exit_game();
                    }
                });
        });

        egui_macroquad::draw()
    }

}

impl Game for GameStruct {
    fn new(
        player_texture: Texture2D, 
        space_object_textures: Vec<Texture2D>,
        game_music: Vec<Sound>,
        game_sounds: Vec<Sound>
    ) -> Self 
    {
        let mut j = Vec::new();
        let number_of_circles = gen_range(20, 30);
        for _ in 0..number_of_circles {
            let texture_chose = gen_range(0, 200);
            let scrap_texture_number: i32;
            if texture_chose > 50 {
                scrap_texture_number = 0;
            }
            else if texture_chose < 50 && texture_chose > 100 {
                scrap_texture_number = 0;
            }
            else {
                scrap_texture_number = 0;
            }
            let scrap_texture: Texture2D = space_object_textures[scrap_texture_number as usize];
            j.push(SpaceObject::new(scrap_texture));
        }
        let mut a = Vec::new();
        let number_of_asteriods = gen_range(5, 10);
        for _ in 0..number_of_asteriods {
            let asteroid_texture: Texture2D = space_object_textures[1];
            a.push(SpaceObject::new(asteroid_texture));
        }
        let mut p = Player::new(player_texture);
        match std::fs::read("data/player.json") {
            Ok(_) => {
                Self{ player: p.load_player(player_texture, "data/player.json".to_string()), scraps: j, asteroids: a, debug: false, paused: false, state: State::MainMenu, previous_state: State::MainMenu, game_music: game_music, game_sounds: game_sounds, music_timer: Timer::new(13.0, true), music_volume: 25.0, sound_volume: 25.0 }
            },
            Err(_) => {
                Self{ player: Player::new(player_texture), scraps: j, asteroids: a, debug: false, paused: false, state: State::MainMenu, previous_state: State::MainMenu, game_music: game_music, game_sounds: game_sounds, music_timer: Timer::new(13.0, true), music_volume: 25.0, sound_volume: 25.0 }
            },
        }
    }

    fn update(&mut self) {
        match self.state {
            State::MainMenu => {
                if self.music_timer.is_timer_done() {
                    let song_choice = gen_range(0, 1);
                    stop_sound(self.game_music[0]);
                    stop_sound(self.game_music[1]);
                    play_sound(self.game_music[song_choice as usize], PlaySoundParams { looped: false, volume: self.music_volume });
                    self.music_timer = Timer::new(13.0, false);
                }
            },
            State::Credits => {},
            State::Options => {},
            State::GameTutorial => {},
            State::Game => {
                if is_key_pressed(KeyCode::B) {
                    self.player.save_player();
                }
                if is_key_pressed(KeyCode::P) || is_key_pressed(KeyCode::Escape) {
                    self.paused = !self.paused;
                }
                if self.paused == false {
                    if is_key_pressed(KeyCode::G) {
                        self.debug = !self.debug;
                    }
                    if self.player.health <= 0 {
                        self.state = State::GameOver;
                    }
                    let speed = calculate_speed(self.player.points);
                    if is_key_down(KeyCode::A) && self.player.get_x() > 0.0 {
                        self.player.move_x(-(speed * get_frame_time()));
                    }
                    if is_key_down(KeyCode::D) && self.player.get_x() < screen_width() - self.player.get_width() {
                        self.player.move_x(speed * get_frame_time());
                    }
                    for touch in touches().iter_mut() {
                        if touch.position.x > self.player.get_x() {
                            self.player.move_x(speed * get_frame_time());
                        }
                        else if touch.position.x < self.player.get_x() {
                            self.player.move_x(-(speed * get_frame_time()));
                        }
                    }
                    for junk in self.scraps.iter_mut() {
                        if junk.position.y > screen_height() {
                            junk.reset();
                        }
                        else if junk.position.overlaps(&self.player.get_rect()) {
                            junk.hit(self.game_sounds[1], self.sound_volume);
                            self.player.collect_point(junk.get_points());
                        }
                        else {
                            junk.move_y(speed / 2.0 * get_frame_time());
                        }
                    }
                    for asteroid in self.asteroids.iter_mut() {
                        if asteroid.position.y > screen_height() {
                            asteroid.reset()
                        }
                        else if asteroid.position.overlaps(&self.player.get_rect()) {
                            asteroid.hit(self.game_sounds[0], self.sound_volume);
                            self.player.hurt(asteroid.get_health());
                        }
                        else {
                            asteroid.move_y(speed / 2.0 * get_frame_time());
                        }
                    }
                }
            },
            State::Shop => {},
            State::GameOver => {},
        }
    }
    
    fn draw(&mut self) {
        match self.state {
            State::MainMenu => {
                self.draw_main_menu();
            },
            State::Credits => {
                clear_background(BLACK);
                let window_frame = egui::containers::Frame{
                    fill: egui::Color32::TRANSPARENT,
                    ..Default::default()
                };
                egui_macroquad::ui(|egui_ctx| {
                    egui::Window::new("egui ❤ macroquad")
                        .title_bar(false)
                        .frame(window_frame)
                        .default_pos(Pos2{ x: (screen_width() / 2.0 + 250.0) / 3.0, y: (screen_height() / 2.0 + 200.0) / 3.0 })
                        .resizable(false)
                        .show(egui_ctx, |ui| {
                            egui_ctx.set_pixels_per_point(3.0);
                            ui.label("Credits");
                            ui.hyperlink_to("Code, Music, and Sound FX made by Anatoliy K.", "https://linktr.ee/anatoliyk05");
                            ui.hyperlink_to("Textures made by @happyghost_fren on Instagram", "https://www.instagram.com/happyghost_fren/");
                            ui.hyperlink_to("Sound FX with JSFXR", "https://sfxr.me/");
                            ui.hyperlink_to("Music made with Beepbox", "https://www.beepbox.co/");
                            ui.hyperlink_to("Built using Macroquad", "https://macroquad.rs");
                            ui.hyperlink_to("and egui", "https://egui.rs");
                            if ui.button("Back").clicked() {
                                self.state = self.previous_state.clone();
                            }
                        });
                });

                egui_macroquad::draw();
            },
            State::GameTutorial => {
                clear_background(BLACK);
                for asteroid in self.asteroids.iter_mut() {
                    let asteroid_parmas = DrawTextureParams{
                        dest_size: Some(Vec2{ x: asteroid.position.w, y: asteroid.position.h}),
                        source: None,
                        rotation: 0.0,
                        flip_x: false,
                        flip_y: false,
                        pivot: None,
                    };
                    draw_texture_ex(asteroid.texture, (screen_width() / 10.0 * 4.5 + (asteroid.position.w / 2.0)) - 150.0, screen_height() / 2.0, WHITE, asteroid_parmas);
                    draw_text("Avoid the Asteroids", (screen_width() / 10.0 * 4.5 + (asteroid.position.w / 2.0)) - 250.0, screen_height() / 2.0 + asteroid.position.h + 25.0, 25.0, WHITE);
                }
                for scrap in self.scraps.iter_mut() {
                    let scrap_parmas = DrawTextureParams{
                        dest_size: Some(Vec2{ x: scrap.position.w, y: scrap.position.h}),
                        source: None,
                        rotation: 0.0,
                        flip_x: false,
                        flip_y: false,
                        pivot: None,
                    };
                    draw_texture_ex(scrap.texture, screen_width() / 10.0 * 4.5 + (scrap.position.w / 2.0), screen_height() / 2.0, WHITE, scrap_parmas);
                    draw_text("Catch the Scraps", screen_width() / 10.0 * 4.5 + (scrap.position.w / 2.0), screen_height() / 2.0 + scrap.position.h + 25.0, 25.0, WHITE);
                }
                let play_rect = Rect { 
                    x: screen_width() / 2.0 - (175.0 / 2.0),
                    y: screen_height() / 10.0 * 7.5,
                    w: 175.0,
                    h: 50.0
                };
                let mut play_button = Button::new(
                    play_rect,
                    String::from("Play Game"),
                    25.0,
                    LIGHTGRAY,
                    BLACK
                );
                play_button.draw();
                if play_button.clicked() {
                    self.state = State::Game;
                }
            },
            State::Game => {
                clear_background(BLACK);

                if self.paused {
                    self.draw_pause();
                }

                for scrap in self.scraps.iter() {
                    //draw_circle(junk.position.x, junk.position.y, junk.position.r, RED);
                    let scrap_parmas = DrawTextureParams{
                        dest_size: Some(Vec2{ x: scrap.position.w, y: scrap.position.h}),
                        source: None,
                        rotation: scrap.rotate,
                        flip_x: false,
                        flip_y: false,
                        pivot: None,
                    };
                    draw_texture_ex(scrap.texture, scrap.position.x, scrap.position.y, WHITE, scrap_parmas);
                    if self.debug {
                        draw_rectangle_lines(scrap.position.x, scrap.position.y, scrap.position.w, scrap.position.h, 5.0, BLUE);
                    }
                }

                for asteroid in self.asteroids.iter() {
                    //draw_circle(junk.position.x, junk.position.y, junk.position.r, RED);
                    let asteroid_parmas = DrawTextureParams{
                        dest_size: Some(Vec2{ x: asteroid.position.w, y: asteroid.position.h}),
                        source: None,
                        rotation: asteroid.rotate,
                        flip_x: false,
                        flip_y: false,
                        pivot: None,
                    };
                    draw_texture_ex(asteroid.texture, asteroid.position.x, asteroid.position.y, WHITE, asteroid_parmas);
                    if self.debug {
                        draw_rectangle_lines(asteroid.position.x, asteroid.position.y, asteroid.position.w, asteroid.position.h, 5.0, BLUE);
                    }
                }

                //draw_rectangle(self.player.position.x, self.player.position.y, self.player.position.w, self.player.position.h, GREEN);
                let player_parmas = DrawTextureParams{
                    dest_size: Some(Vec2{ x: self.player.get_width(), y: self.player.get_height()}),
                    source: None,
                    rotation: 0.0,
                    flip_x: false,
                    flip_y: false,
                    pivot: None,
                };
                draw_texture_ex(self.player.texture, self.player.get_x(), self.player.get_y(), WHITE, player_parmas);
                if self.debug {
                    draw_rectangle_lines(self.player.get_x(), self.player.get_y(), self.player.get_width(), self.player.get_height(), 5.0, BLUE);
                }

                draw_text(&format!("Points: {}", self.player.points).to_owned(), 50.0, 50.0, 25.0, WHITE);

                draw_text(&format!("Health: {}", self.player.health).to_owned(), screen_width() / 10.0 * 8.5, 50.0, 25.0, WHITE);

                if self.debug {
                    draw_text(&format!("FPS: {}", get_fps()).to_owned(), screen_width() / 2.0, 50.0, 25.0, WHITE);
                }
                if self.paused {
                    egui_macroquad::draw();
                }
            },
            State::Options => {
                let window_frame = egui::containers::Frame{
                    fill: egui::Color32::TRANSPARENT,
                    ..Default::default()
                };
                egui_macroquad::ui(|egui_ctx| {
                    egui::Window::new("egui ❤ macroquad")
                        .title_bar(false)
                        .frame(window_frame)
                        .default_pos(Pos2{ x: (screen_width() / 2.0 + 250.0) / 3.0, y: (screen_height() / 2.0 + 200.0) / 3.0 })
                        .resizable(false)
                        .show(egui_ctx, |ui| {
                            egui_ctx.set_pixels_per_point(3.0);
                            ui.label("Options");
                            ui.add(egui::widgets::Slider::new(&mut self.music_volume, 0.0..=100.0).text("Music Volume"));
                            ui.add(egui::widgets::Slider::new(&mut self.sound_volume, 0.0..=100.0).text("Sound Volume"));
                            if ui.button("Back").clicked() {
                                self.state = self.previous_state.clone();
                            }
                        });
                });

                egui_macroquad::draw();
            },
            State::Shop => {
                clear_background(BLACK);
                let window_frame = egui::containers::Frame{
                    fill: egui::Color32::TRANSPARENT,
                    ..Default::default()
                };
                egui_macroquad::ui(|egui_ctx| {
                    egui::Window::new("egui ❤ macroquad")
                        .title_bar(false)
                        .frame(window_frame)
                        .default_pos(Pos2{ x: (screen_width() / 2.0 + 250.0) / 3.0, y: (screen_height() / 2.0 + 200.0) / 3.0 })
                        .resizable(false)
                        .show(egui_ctx, |ui| {
                            egui_ctx.set_pixels_per_point(3.0);
                            ui.label("Shop");
                            ui.label(format!("Points {} Coins {}", self.player.points, self.player.coins));
                            if ui.button("Convert Points to Coins").clicked() {
                                let new_coins = self.player.points / 10;
                                self.player.coins += new_coins;
                                self.player.points -= new_coins;
                            }
                            let player_parmas = DrawTextureParams{
                                dest_size: Some(Vec2{ x: self.player.get_width(), y: self.player.get_height()}),
                                source: None,
                                rotation: 0.0,
                                flip_x: false,
                                flip_y: false,
                                pivot: None,
                            };
                            draw_texture_ex(self.player.texture, self.player.get_x(), self.player.get_y(), WHITE, player_parmas);
                            if ui.button("Back").clicked() {
                                self.state = State::Game;
                            }
                        });
                });
        
                egui_macroquad::draw();
            },
            State::GameOver => {
                clear_background(BLACK);
                let window_frame = egui::containers::Frame{
                    fill: egui::Color32::TRANSPARENT,
                    ..Default::default()
                };
                egui_macroquad::ui(|egui_ctx| {
                    egui::Window::new("egui ❤ macroquad")
                        .title_bar(false)
                        .frame(window_frame)
                        .default_pos(Pos2{ x: (screen_width() / 2.0 + 250.0) / 3.0, y: (screen_height() / 2.0 + 200.0) / 3.0 })
                        .resizable(false)
                        .show(egui_ctx, |ui| {
                            egui_ctx.set_pixels_per_point(3.0);
                            ui.label(&format!("Game Over! You has {} points!", self.player.points).to_owned());
                            if ui.button("Play Again").clicked() {
                                self.player.points = 0;
                                self.player.health = 5;
                                for space_scraps in self.scraps.iter_mut() {
                                    space_scraps.reset();
                                }
                                for asteroids in self.asteroids.iter_mut() {
                                    asteroids.reset();
                                }
                                self.state = State::Game;
                            }
                            if ui.button("Quit").clicked() {
                                exit_game();
                            }
                        });
                });

                egui_macroquad::draw();
            },
        }

    }

}

fn window_conf() -> Conf {
    Conf {
        window_title: "Space Cleanup".to_owned(),
        high_dpi: true,
        window_height: 720,
        window_width: 1280,
        ..Default::default()
    }
}

#[cfg(not(debug_assertions))] 
#[macroquad::main(window_conf)]
async fn main() {
    let player_path: &str;
    let scraps_path: &str;
    let asteroid_path: &str;
    let song1_path: &str;
    let song2_path: &str;
    let sound1_path: &str;
    let sound2_path: &str;
    if std::env::consts::OS.to_lowercase() == "macos".to_lowercase() {
        player_path = "../Resources/res/player.png";
        scraps_path = "../Resources/res/junk1.png";
        asteroid_path = "../Resources/res/asteroid.png";
        song1_path = "../Resources/res/music/song1.wav";
        song2_path = "../Resources/res/music/song2.wav";
        sound1_path = "../Resources/res/sounds/hit.wav";
        sound2_path = "../Resources/res/sounds/pickup.wav";
    }
    else {
        player_path = "res/player.png";
        scraps_path = "res/junk1.png";
        asteroid_path = "res/asteroid.png";
        song1_path = "res/music/song1.wav";
        song2_path = "res/music/song2.wav";
        sound1_path = "res/sounds/hit.wav";
        sound2_path = "res/sounds/pickup.wav";
    }

    let player_image = init_texture(player_path).await;

    let mut junk_texture_list = Vec::new();
    junk_texture_list.push(init_texture(scraps_path).await);
    junk_texture_list.push(init_texture(asteroid_path).await);

    let mut game_music = Vec::new();
    game_music.push(init_sound(song1_path).await);
    game_music.push(init_sound(song2_path).await);

    let mut game_sounds = Vec::new();
    game_sounds.push(init_sound(sound1_path).await);
    game_sounds.push(init_sound(sound2_path).await);

    let mut main_game = GameStruct::new(player_image, junk_texture_list, game_music, game_sounds);

    loop { 

        main_game.run();

        next_frame().await

    }
}

#[cfg(debug_assertions)]
#[macroquad::main(window_conf)]
async fn main() {
    let player_path = "res/player.png";
    let scraps_path = "res/junk1.png";
    let asteroid_path = "res/asteroid.png";
    let song1_path = "res/music/song1.wav";
    let song2_path = "res/music/song2.wav";
    let sound1_path = "res/sounds/hit.wav";
    let sound2_path = "res/sounds/pickup.wav";

    let player_image = init_texture(player_path).await;

    let mut junk_texture_list = Vec::new();
    junk_texture_list.push(init_texture(scraps_path).await);
    junk_texture_list.push(init_texture(asteroid_path).await);

    let mut game_music = Vec::new();
    game_music.push(init_sound(song1_path).await);
    game_music.push(init_sound(song2_path).await);

    let mut game_sounds = Vec::new();
    game_sounds.push(init_sound(sound1_path).await);
    game_sounds.push(init_sound(sound2_path).await);

    let mut main_game = GameStruct::new(player_image, junk_texture_list, game_music, game_sounds);

    loop { 

        main_game.run();

        next_frame().await

    }
}
