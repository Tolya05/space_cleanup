use macroquad::prelude::{Rect, Texture2D, screen_height, screen_width};
use crate::{object::Object, game};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{fs::{File, create_dir}, io::{Write, Read}};

pub struct Player {
    position: Rect,
    pub health: i32,
    pub points: i32,
    pub coins: i32,
    pub texture: Texture2D,
}

#[derive(Debug, Deserialize, Serialize)]
struct Vector2D {
    x: f32,
    y: f32,
}

#[derive(Debug, Deserialize, Serialize)]
struct PlayerJson {
    position: Vector2D,
    coins: i32,
}

impl PlayerJson {
    fn new(x_pos: f32, y_pos: f32, coins: i32) -> Self {
        Self { position: Vector2D { x: x_pos, y: y_pos }, coins: coins }
    }
}

impl Player {
    pub fn get_coins(&self) -> i32 {
        self.coins.clone()
    }

    pub fn hurt(&mut self, damage: i32) {
        self.health += damage;
    }

    pub fn collect_point(&mut self, points: i32) {
        self.points += points;
    }

    pub fn save_player(&self) {
        let x_pos = self.get_x();
        let y_pos = self.get_y();
        let coins = self.get_coins();
        let player_json = PlayerJson::new( x_pos, y_pos, coins );
        let mut player_data = String::new();
        match serde_json::to_string(&player_json) {
            Ok(data) => {
                player_data = data.clone();
            },
            Err(error) => {
                println!("An Error has occured: {}", error);
                game::exit_game();
            },
        }
        match create_dir("data") {
            Ok(_) => {
                println!("Data directory created");
            },
            Err(error) => {
                println!("{}", error);
            }
        }
        let json_file_init = File::create("data/player.json");
        let mut json_file: File;
        match json_file_init {
            Ok(file) => {
                json_file = file;
                println!("Saved data");
            },
            Err(error) => {
                println!("Cannot Create File {}", error);
                game::exit_game();
            }
        }
        match json_file.write_all(player_data.as_bytes()) {
            Ok(_) => {/*println!("File Written!")*/},
            Err(error) => println!("Error {}", error),
        }
    }

    pub fn load_player(&mut self, texture: Texture2D, path: String) -> Player {
        let mut file = File::open(path).expect("Failed to open file");

        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read file");

        let data: PlayerJson = serde_json::from_str(&contents).expect("Failed to deserialize JSON");
        Player { position: Rect { x: data.position.x, y: data.position.x, w: 75.0, h: 125.0 }, health: 5, points: 0, coins: data.coins, texture: texture }
    }
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