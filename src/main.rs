pub mod tile;
pub mod world;
pub mod config;
pub mod player;
use raylib::prelude::*;
use crate::tile::Tile;
use crate::world::World;
use crate::config::Config;
use crate::player::Player;

fn main() {
    let world = World::new("./world.txt");

    // get the position of the player and set it to an empty tile
    let player_pos = world.player_pos;
    let config = Config::load_config();
    let player: Player = Player::new(player_pos);
    let (mut rl, thread) = raylib::init()
        .size(config.window_width, config.window_height)
        .title(&config.window_name)
        .build();
    
    
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);


        // Draw the world as squares with a size of 32x32, with a 1px border and the colour of the tile in the world file
        for (position, tile) in world.tiles.iter() {
            d.draw_rectangle(
                position.0 * 32,
                position.1 * 32,
                32,
                32,
                match tile {
                    Tile::Empty => config.empty_colour(),
                    Tile::Player => config.player_colour(),
                    Tile::Wall => config.wall_colour()
                },
            );
            // draw outlines for the tiles
            if config.draw_outlines {
                // get the tile that the mouse is over
                let mouse_pos = d.get_mouse_position();
                let mouse_tile = ((mouse_pos.x / 32.0 ) as i32, (mouse_pos.y / 32.0) as i32);
                // if the mouse is over a tile, draw a border around it
                if mouse_tile == *position {
                    d.draw_rectangle_lines(
                        position.0 * 32,
                        position.1 * 32,
                        32,
                        32,
                        config.outline_colour()
                    );
                }
            }
        }
        d.clear_background(Color::BLACK);
    }
}