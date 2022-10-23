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
        if config.debug {
            draw_outlines(position, config, d);
            draw_lines(world, d);
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

pub fn draw_lines(world: &World, d: &mut RaylibDrawHandle) {
    // get all enemies on the board
    let mut enemy_positions: Vec<(i32, i32)> = Vec::new();
    for (pos, tile) in world.tiles.iter() {
        if *tile == Tile::Enemy {
            enemy_positions.push(*pos);
        }
    }
    // draw lines from the enemies to the player
    for enemy in enemy_positions {
        d.draw_line(
            enemy.0 * 32 + 16,
            enemy.1 * 32 + 16,
            world.player.pos.0 * 32 + 16,
            world.player.pos.1 * 32 + 16,
            Color::RED
        );
    }


}

pub fn draw_time(time: f32, config: &Config, d: &mut RaylibDrawHandle) {
    d.draw_text(&format!("Time: {}", (config.default_time - time) as i32), 30, 30, 20, config.timer_colour());
}

#[derive(PartialEq, Debug)]
pub enum EndScreenSignal {
    Restart,
    Quit,
    Wait
}

pub fn end_sceen(config: &Config, d: &mut RaylibDrawHandle, pos: (f32, f32)) -> EndScreenSignal {
    d.draw_text("Ran out of time", pos.0 as i32, pos.1 as i32, 50, config.gui_colour());
    // restart and quit button
    // TODO: fix all positioning and add config items for colours, sizes and positions

    if d.gui_button(Rectangle::new(pos.0, pos.1 + 50.0, 200.0, 50.0), Some(rstr!("Quit"))) {
        return EndScreenSignal::Quit;
    } else if d.gui_button(Rectangle::new(pos.0+200.0, pos.1+50.0, 200.0, 50.0), Some(rstr!("Restart"))) {
        return EndScreenSignal::Restart;
    }
    EndScreenSignal::Wait
}