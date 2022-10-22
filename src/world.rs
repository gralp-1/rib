// A grid of tiles
use crate::tile::Tile;
use std::collections::HashMap;
pub struct World {
    pub tiles: HashMap<(i32, i32), Tile>,
}

impl World {
    fn get_tile(&self, x: i32, y: i32) -> Option<&Tile> {
        self.tiles.get(&(x, y))
    }

    fn set_tile(&mut self, x: i32, y: i32, tile: Tile) {
        self.tiles.insert((x, y), tile);
    }
    pub fn print_tiles(&self) {
        for y in 0..10 {
            for x in 0..10 {
                match self.get_tile(x, y) {
                    Some(tile) => print!("{:?}", tile),
                    None => print!("None"),
                }
            }
            println!();
        }
    }
    pub fn new(path: &str) -> Self {
        // Load world from file at path
        // Return a new world
        let mut tiles: HashMap<(i32, i32), Tile> = HashMap::new();
        // load file
        let world_data = std::fs::read_to_string(path).expect("Unable to read file");
        // if there is more than one character P in the file, panic
        if world_data.matches('P').count() > 1 {
            panic!("More than one player in world file");
        }
        // parse file
        let mut x = 0;
        let mut y = 0;
        for line in world_data.lines() {
            for c in line.chars() {
                match c {
                    ' ' => tiles.insert((x,y), Tile::Empty),
                    'P' => tiles.insert((x,y), Tile::Player),
                    'W' => tiles.insert((x,y), Tile::Wall),
                    _ => panic!("Invalid character in world file at {}:{}", y+1, x+1)
                };
                x += 1;
            }
            x = 0;
            y += 1;
        }
        World {
            tiles
        }
    }
}