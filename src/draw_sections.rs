use crate::world::World;
use crate::config::Config;
use crate::tile::Tile;
use raylib::prelude::*;

pub fn draw_tiles(world: &mut World, config: &Config, d: &mut RaylibDrawHandle) {
    for (position, tile) in world.tiles.iter() {
        d.draw_rectangle(
            position.0 * 32,
            position.1 * 32,
            32,
            32,
            match tile {
                Tile::Empty => config.empty_colour(),
                Tile::Player => config.player_colour(),
                Tile::Wall => config.wall_colour(),
                Tile::Enemy => config.enemy_colour()
            },
        );
        if config.draw_outlines {
            draw_outlines(position, config, d);
        }
    }
}
fn draw_outlines(position: &(i32, i32), config: &Config, d: &mut RaylibDrawHandle) {
    // draw outlines for the tiles
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

pub fn draw_time(time: f32, config: &Config, d: &mut RaylibDrawHandle) {
    d.draw_text(&format!("Time: {}", (config.default_time - time) as i32), 30, 30, 20, config.timer_colour());
}

pub fn end_sceen(config: &Config, d: &mut RaylibDrawHandle, pos: (f32, f32)) -> bool {
    d.draw_text("Ran out of time", pos.0 as i32, pos.1 as i32, 50, config.gui_colour());
    // restart and quit button
    if d.gui_button(Rectangle::new(pos.0, pos.1 + 50.0, 100.0, 50.0), Some(rstr!("Quit"))) {
        false
    } else {
        true
    }

}