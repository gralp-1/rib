use serde::Serialize;

use crate::world::World;

pub struct Player {
    pub pos: (i32, i32),
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Player {
    pub fn new(pos: (i32, i32)) -> Self {
        Player {
            pos
        }
    }
    pub fn move_player(&mut self, direction: Direction, mut world: World) -> World {
        // check if the tile in the direction is empty
        // if it is, move the player
        // if it isn't, do nothing
        let (x, y) = self.pos;
        match direction {
            Direction::Up => {
                if world.tiles.get(&(x, y-1)) == Some(&crate::tile::Tile::Empty) {
                    self.pos = (x, y-1);
                    world.tiles.insert(world.player_pos, crate::tile::Tile::Empty);
                    world.tiles.insert(self.pos, crate::tile::Tile::Player);
                    world.player_pos = self.pos;
                }
                world
            },
            Direction::Down => {
                if world.tiles.get(&(x, y+1)) == Some(&crate::tile::Tile::Empty) {
                    self.pos = (x, y+1);
                    world.tiles.insert(world.player_pos, crate::tile::Tile::Empty);
                    world.tiles.insert(self.pos, crate::tile::Tile::Player);
                    world.player_pos = self.pos;
                }
                world
            },
            Direction::Left => {
                if world.tiles.get(&(x-1, y)) == Some(&crate::tile::Tile::Empty) {
                    self.pos = (x-1, y);
                    world.tiles.insert(world.player_pos, crate::tile::Tile::Empty);
                    world.tiles.insert(self.pos, crate::tile::Tile::Player);
                    world.player_pos = self.pos;
                }
                world
            },
            Direction::Right => {
                if world.tiles.get(&(x+1, y)) == Some(&crate::tile::Tile::Empty) {
                    self.pos = (x+1, y);
                    world.tiles.insert(world.player_pos, crate::tile::Tile::Empty);
                    world.tiles.insert(self.pos, crate::tile::Tile::Player);
                    world.player_pos = self.pos;
                }
                world
            },
        }
    }
}