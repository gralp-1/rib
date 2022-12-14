use rand::prelude::Distribution;

use serde::Serialize;

use crate::world::World;

pub struct Player {
    pub pos: (i32, i32),
    pub health: i32,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Distribution<Direction> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0..4) {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            _ => panic!("Invalid direction"),
        }
    }
}

impl Player {
    pub fn new(pos: (i32, i32)) -> Self {
        Player {
            pos,
            health: 50,
        }
    }
    
    pub fn move_player(&mut self, direction: Direction, mut world: World) -> World {
        puffin::profile_function!("move_player");
        // check if the tile in the direction is empty
        // if it is, move the player
        // if it isn't, do nothing
        let (x, y) = self.pos;
        match direction {
            Direction::Up => {
                if world.tiles.get(&(x, y-1)) == Some(&crate::tile::Tile::Empty) {
                    self.pos = (x, y-1);
                    world.tiles.insert(world.player.pos , crate::tile::Tile::Empty);
                    world.tiles.insert(self.pos, crate::tile::Tile::Player);
                    world.player.pos = self.pos;
                }
                world
            },
            Direction::Down => {
                if world.tiles.get(&(x, y+1)) == Some(&crate::tile::Tile::Empty) {
                    self.pos = (x, y+1);
                    world.tiles.insert(world.player.pos , crate::tile::Tile::Empty);
                    world.tiles.insert(self.pos, crate::tile::Tile::Player);
                    world.player.pos = self.pos;
                }
                world
            },
            Direction::Left => {
                if world.tiles.get(&(x-1, y)) == Some(&crate::tile::Tile::Empty) {
                    self.pos = (x-1, y);
                    world.tiles.insert(world.player.pos , crate::tile::Tile::Empty);
                    world.tiles.insert(self.pos, crate::tile::Tile::Player);
                    world.player.pos = self.pos;
                }
                world
            },
            Direction::Right => {
                if world.tiles.get(&(x+1, y)) == Some(&crate::tile::Tile::Empty) {
                    self.pos = (x+1, y);
                    world.tiles.insert(world.player.pos, crate::tile::Tile::Empty);
                    world.tiles.insert(self.pos, crate::tile::Tile::Player);
                    world.player.pos = self.pos;
                }
                world
            },
        }
    }
}