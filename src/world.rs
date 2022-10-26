use puffin::{profile_function, profile_scope};

// A grid of tiles
use crate::{tile::Tile, config::Config, player::{Direction, Player}};
use std::collections::HashMap;
pub struct World {
    pub tiles: HashMap<(i32, i32), Tile>,
    pub config: Config,
    pub player: Player,
    pub enemy_positions: Vec<(i32, i32)>,
}

impl World {

    pub fn new(config: Config) -> Self {
        profile_scope!("world_new");
        // Load world from file at path
        // Return a new world
        let mut tiles: HashMap<(i32, i32), Tile> = HashMap::new();
        // load file
        let world_data = std::fs::read_to_string(&config.world_file).expect("Unable to read file");
        // if there is more than one character P in the file, panic
        if world_data.matches('P').count() != 1 {
            panic!("There must be exactly one player in the world file");
        }
        // parse file
        let mut x = 0;
        let mut y = 0;
        let mut player_pos: (i32, i32) = (0, 0);
        let mut enemy_positions: Vec<(i32, i32)> = Vec::new();
        for line in world_data.lines() {
            for c in 0..(config.window_width / 32) {
                match line.chars().nth(c as usize) {
                    Some(' ') => tiles.insert((x,y), Tile::Empty),
                    Some('E') => {enemy_positions.push((x, y)); tiles.insert((x,y), Tile::Enemy)},
                    Some('P') => {player_pos = (x,y); tiles.insert((x,y), Tile::Player)},
                    Some('W') => tiles.insert((x,y), Tile::Wall),
                    _ => tiles.insert((x,y), Tile::Empty),
                };
                x += 1;
            }
            x = 0;
            y += 1;
        }

        World {
            tiles,
            player: Player::new(player_pos),
            enemy_positions,
            config
        }
    }
    pub fn save_world(&self) {
        profile_function!("save_world");
        // save the world to file
        // iterate through tiles, and write to file line by line
        let mut world_data = String::new();
        for y in 0..(self.config.window_height / 32) {
            for x in 0..(self.config.window_width / 32) {
                match self.tiles.get(&(x,y)) {
                    Some(Tile::Empty) => world_data.push(' '),
                    Some(Tile::Player) => world_data.push('P'),
                    Some(Tile::Wall) => world_data.push('W'),
                    Some(Tile::Enemy) => world_data.push('E'),
                    _ => world_data.push(' '),
                }
            }
            world_data.push('\n');
        }

        std::fs::write(&self.config.world_file, world_data).expect("Unable to write file");
        info!("World saved to {}", &self.config.world_file);
    }

    pub fn update_enemy_positions(&mut self) {
        info!("Updating enemy positions");
        profile_function!("update_enemy_positions");
        // move the enemies in any direction by one tile
        // if the enemy is next to the player, kill the player

        // get all the enemy positions
        // move the enemies
        let mut enemy_positions_vec= self.enemy_positions.clone();
        for (x,y) in &self.enemy_positions {
            // get the direction to move in
            let direction: Direction = rand::random();
            // check if the tile in that direction is empty
            let new_pos: (i32, i32) = match direction {
                Direction::Up =>    (*x,   *y-1),
                Direction::Down =>  (*x,   *y+1),
                Direction::Left =>  (*x-1, *y),
                Direction::Right => (*x+1, *y),
            };
            match self.tiles.get(&new_pos) {
                Some(Tile::Empty) => {
                    // move the enemy
                    self.tiles.insert((*x,*y), Tile::Empty);
                    self.tiles.insert(new_pos, Tile::Enemy);
                    enemy_positions_vec.retain(|&pos| pos != (*x,*y));
                    enemy_positions_vec.push(new_pos);
                },
                // Some(Tile::Player) => {
                //     // kill the player
                //     self.tiles.insert((x,y), Tile::Empty);
                //     self.tiles.insert(new_pos, Tile::Enemy);
                //     self.tiles.insert(self.player_pos, Tile::Empty);
                //     self.player_pos = (0,0);
                // },
                _ => {},
            };
        }
        self.enemy_positions = enemy_positions_vec;
    }
}