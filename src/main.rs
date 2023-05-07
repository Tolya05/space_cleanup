use macroquad::{prelude::*, rand::*, math::Rect, audio::{load_sound, Sound, PlaySoundParams, play_sound, play_sound_once}};
use egui_macroquad::egui::{self, Pos2};

struct Timer {
    start_time: f64,
    life_time:f64,
    first_time: bool,
}

impl Timer {
    fn new(time: f64, intial: bool) -> Timer {
        Timer{ start_time: get_time(), life_time: time, first_time: intial }
    }

    fn is_timer_done(&self) -> bool {
        let current_time = get_time();
        let time_elapsed = current_time - self.start_time;
        if time_elapsed == self.life_time {
            true
        }
        else {
            false
        }
    }
}

#[derive(Clone, Copy)]
enum State {
    New,
    Credits,
    Main,
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

struct Player {
    position: Rect,
    health: i32,
    points: i32,
    coins: i32,
    texture: Texture2D,
}

struct SpaceObject {
    position: Rect,
    points: i32,
    texture: Texture2D,
    rotate: f32,
    health: i32
}

struct Game {
    player: Player,
    space_junk: Vec<SpaceObject>,
    asteroids: Vec<SpaceObject>,
    hitbox: bool,
    paused: bool,
    state: State,
    previous_state: State,
    game_music: Vec<Sound>,
    game_sounds: Vec<Sound>,
    music_timer: Timer,
}

impl Game {
    fn new(rectangle_position: Rect, player_texture: Texture2D, space_junk_textures: Vec<Texture2D>, game_music: Vec<Sound>, game_sounds: Vec<Sound>) -> Game {
        let mut j = Vec::new();
        let number_of_circles = gen_range(20, 30);
        for _ in 0..number_of_circles {
            let texture_chose = gen_range(0, 100);
            let junk_texture_number: i32;
            if texture_chose > 50 {
                junk_texture_number = 0;
            }
            else {
                junk_texture_number = 1;
            }
            let junk_texture: Texture2D = space_junk_textures[junk_texture_number as usize];
            j.push(SpaceObject{ position: Rect { x: gen_range(0.0, screen_width() - 64.0), y: gen_range(0.0, 50.0), w: 64.0, h: 64.0 }, points: gen_range(1, 5), texture: junk_texture, rotate: gen_range(0.0, 360.0), health: 0 });
        }
        let mut a = Vec::new();
        let number_of_asteriods = gen_range(5, 10);
        for _ in 0..number_of_asteriods {
            let junk_texture: Texture2D = space_junk_textures[2];
            a.push(SpaceObject{ position: Rect { x: gen_range(0.0, screen_width() - 64.0), y: gen_range(0.0, 50.0), w: 64.0, h: 64.0 }, points: gen_range(1, 5), texture: junk_texture, rotate: gen_range(0.0, 360.0), health: -1 });
        }
        let p = Player{ position: rectangle_position, health: 5, points: 0, coins: 0, texture: player_texture };
        Game{ player: p, space_junk: j, asteroids: a, hitbox: false, paused: false, state: State::New, previous_state: State::New, game_music: game_music, game_sounds: game_sounds, music_timer: Timer::new(13.0, true) }
    }

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
                        self.previous_state = State::Main;
                        self.state = State::Credits;
                    }
                    if ui.button("Quit").clicked() {
                        std::process::exit(0);
                    }
                });
        });
    }

    fn update(&mut self) {
        match self.state {
            State::New => {
                if self.music_timer.is_timer_done() || self.music_timer.first_time {
                    let song_choice = gen_range(0, 1);
                    play_sound_once(self.game_music[song_choice as usize]);
                    self.music_timer = Timer::new(13.0, false);
                }
            },
            State::Credits => {},
            State::Main => {
                if is_key_pressed(KeyCode::P) || is_key_pressed(KeyCode::Escape) {
                    self.paused = !self.paused;
                }
                if self.paused == false {
                    if is_key_pressed(KeyCode::G) {
                        self.hitbox = !self.hitbox;
                    }
                    if self.player.health <= 0 {
                        self.state = State::GameOver;
                    }
                    let speed = calculate_speed(self.player.points);
                    if is_key_down(KeyCode::A) && self.player.position.x > 0.0 {
                        self.player.position.x -= speed * get_frame_time();
                    }
                    if is_key_down(KeyCode::D) && self.player.position.x < screen_width() - self.player.position.w {
                        self.player.position.x += speed * get_frame_time();
                    }
                    for junk in self.space_junk.iter_mut() {
                        if junk.position.y > screen_height() {
                            junk.position.x = gen_range(0.0, screen_width());
                            junk.position.y = gen_range(0.0, 50.0);
                        }
                        else if junk.position.overlaps(&self.player.position) {
                            play_sound_once(self.game_sounds[1]);
                            junk.position.x = gen_range(0.0, screen_width() - junk.position.w);
                            junk.position.y = gen_range(0.0, 50.0);
                            junk.rotate = gen_range(0.0, 360.0);
                            self.player.points += junk.points;
                        }
                        else {
                            junk.position.y += speed / 2.0 * get_frame_time();
                        }
                    }
                    for asteroid in self.asteroids.iter_mut() {
                        if asteroid.position.y > screen_height() {
                            asteroid.position.x = gen_range(0.0, screen_width());
                            asteroid.position.y = gen_range(0.0, 50.0);
                        }
                        else if asteroid.position.overlaps(&self.player.position) {
                            play_sound_once(self.game_sounds[0]);
                            asteroid.position.x = gen_range(0.0, screen_width() - asteroid.position.w);
                            asteroid.position.y = gen_range(0.0, 50.0);
                            asteroid.rotate = gen_range(0.0, 360.0);
                            self.player.health += asteroid.health;
                        }
                        else {
                            asteroid.position.y += speed / 2.0 * get_frame_time();
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
            State::New => {
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
                                self.state = State::Main;
                            }
                            if ui.button("Credits").clicked() {
                                self.previous_state = State::New;
                                self.state = State::Credits;
                            }
                            if ui.button("Quit").clicked() {
                                std::process::exit(0);
                            }
                        });
                });

                egui_macroquad::draw()
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
                            ui.label("Game made by Anatoliy K.");
                            ui.label("Music made by Anatoliy K.");
                            ui.label("Textures made by Sadie");
                            if ui.button("Back").clicked() {
                                self.state = self.previous_state.clone();
                            }
                        });
                });

                egui_macroquad::draw();
            },
            State::Main => {
                clear_background(BLACK);

                if self.paused {
                    self.draw_pause();
                }

                for junk in self.space_junk.iter() {
                    //draw_circle(junk.position.x, junk.position.y, junk.position.r, RED);
                    let junk_parmas = DrawTextureParams{
                        dest_size: Some(Vec2{ x: junk.position.w, y: junk.position.h}),
                        source: None,
                        rotation: junk.rotate,
                        flip_x: false,
                        flip_y: false,
                        pivot: None,
                    };
                    draw_texture_ex(junk.texture, junk.position.x, junk.position.y, WHITE, junk_parmas);
                    if self.hitbox {
                        draw_rectangle_lines(junk.position.x, junk.position.y, junk.position.w, junk.position.h, 5.0, BLUE);
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
                    if self.hitbox {
                        draw_rectangle_lines(asteroid.position.x, asteroid.position.y, asteroid.position.w, asteroid.position.h, 5.0, BLUE);
                    }
                }

                //draw_rectangle(self.player.position.x, self.player.position.y, self.player.position.w, self.player.position.h, GREEN);
                let player_parmas = DrawTextureParams{
                    dest_size: Some(Vec2{ x: self.player.position.w, y: self.player.position.h}),
                    source: None,
                    rotation: 0.0,
                    flip_x: false,
                    flip_y: false,
                    pivot: None,
                };
                draw_texture_ex(self.player.texture, self.player.position.x, self.player.position.y, WHITE, player_parmas);
                if self.hitbox {
                    draw_rectangle_lines(self.player.position.x, self.player.position.y, self.player.position.w, self.player.position.h, 5.0, BLUE);
                }

                draw_text(&format!("Points: {}", self.player.points).to_owned(), 50.0, 50.0, 25.0, WHITE);

                draw_text(&format!("Health: {}", self.player.health).to_owned(), screen_width() / 10.0 * 8.5, 50.0, 25.0, WHITE);

                draw_text(&format!("FPS: {}", get_fps()).to_owned(), screen_width() / 2.0, 50.0, 25.0, WHITE);
                
                if self.paused {
                    egui_macroquad::draw();
                }
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
                                dest_size: Some(Vec2{ x: self.player.position.w, y: self.player.position.h}),
                                source: None,
                                rotation: 0.0,
                                flip_x: false,
                                flip_y: false,
                                pivot: None,
                            };
                            draw_texture_ex(self.player.texture, self.player.position.x, self.player.position.y, WHITE, player_parmas);
                            if ui.button("Back").clicked() {
                                self.state = State::Main;
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
                                self.state = State::Main;
                            }
                            if ui.button("Quit").clicked() {
                                std::process::exit(0);
                            }
                        });
                });

                egui_macroquad::draw();
            },
        }

    }

    fn run(&mut self) {
        self.update();
        self.draw();
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Space Cleanup".to_owned(),
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let player_texture = load_texture("res/player.png").await.unwrap();

    let mut junk_texture = Vec::new();

    junk_texture.push(load_texture("res/junk1.png").await.unwrap());
    junk_texture.push(load_texture("res/junk2.png").await.unwrap());
    junk_texture.push(load_texture("res/asteroid.png").await.unwrap());

    let mut game_music = Vec::new();
    
    game_music.push(load_sound("res/music/space_cleanup_song1.wav").await.unwrap());

    game_music.push(load_sound("res/music/space_cleanup_song2.wav").await.unwrap());

    let mut game_sounds = Vec::new();
    
    game_sounds.push(load_sound("res/sounds/hit.wav").await.unwrap());

    game_sounds.push(load_sound("res/sounds/pickup.wav").await.unwrap());

    let mut main_game = Game::new(Rect { x: screen_width() / 2.0, y: screen_height() / 10.0 * 7.5, w: 75.0, h: 125.0 }, player_texture, junk_texture, game_music, game_sounds);

    loop { 

        main_game.run();

        next_frame().await

    }
}
