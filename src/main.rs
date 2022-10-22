pub mod tile;
pub mod world;
use raylib::prelude::*;
use crate::tile::Tile;
use crate::world::World;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("huh")
        .build();
    // Load world from ./world.txt
    // print current working directory
    let world = World::new("./world.txt");
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        // is q pressed down?
        if d.is_key_down(raylib::consts::KeyboardKey::KEY_Q) {
            break;
        }
        if d.is_key_down(raylib::consts::KeyboardKey::KEY_W) {
            
        }

        // Draw the world as squares with a size of 32x32, with a 1px border and the colour of the tile in the world file
        for (position, tile) in world.tiles.iter() {
            d.draw_rectangle(
                position.0 * 32,
                position.1 * 32,
                32,
                32,
                match tile {
                    Tile::Empty => Color::BLACK,
                    Tile::Player => Color::RED,
                    Tile::Wall => Color::WHITE,
                },
            );
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
                    Color::BLUE,
                );
            }
        }
        d.clear_background(Color::BLACK);
    }
}