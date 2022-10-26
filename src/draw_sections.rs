use crate::world::World;
use crate::config::Config;
use crate::tile::Tile;
use puffin::{profile_function, profile_scope};
use raylib::prelude::*;

pub fn draw_tiles(world: &World, config: &Config, d: &mut RaylibDrawHandle) {
    profile_function!("draw_tiles");
    for (position, tile) in world.tiles.iter() {
        profile_scope!("draw_tiles_loop");
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
            profile_scope!("draw_call_others");
            draw_tile_outline(position, config, d);
            draw_lines(&world, d);
            draw_closest_enemy(&world, config, d);
        }
    }
}
fn draw_tile_outline(position: &(i32, i32), config: &Config, d: &mut RaylibDrawHandle) {
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

pub fn draw_closest_enemy(world: &World, config: &Config, d: &mut RaylibDrawHandle) {
        profile_function!("draw_closest_enemy");
        // get all the enemy positions
        profile_scope!("get_enemy_positions");
        // find the nearest enemy
        let mut nearest_enemy: (i32, i32) = (0,0);
        let mut nearest_distance: f32 = 100000.0;
        profile_scope!("calc_dists");
        for (x,y) in &world.enemy_positions {
            profile_scope!("calc_dist");
            let distance = (((*x as f32 - (world.player.pos.0) as f32).powf(2.0) + (*y as f32 - (world.player.pos.1 as f32)).powf(2.0)) as f32).sqrt();
            if distance < nearest_distance as f32 {
                nearest_distance = distance as f32;
                nearest_enemy = (*x as i32, *y as i32);
            }
        }
        // highlight the tile around the nearest enemy
        profile_scope!("draw_circ");
        d.draw_circle(nearest_enemy.0 * 32 + 16, nearest_enemy.1 * 32 + 16, 16.0, config.outline_colour());
}

pub fn draw_lines(world: &World, d: &mut RaylibDrawHandle) {
    profile_function!("draw_lines");
    // get all enemies on the board
    // draw lines from the enemies to the player
    profile_scope!("draw_lines");
    for enemy in &world.enemy_positions {
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
    profile_function!("draw_time");
    d.draw_text(&format!("Time: {}", (config.default_time - time) as i32), 30, 30, 20, config.timer_colour());
}

#[derive(PartialEq, Debug)]
pub enum EndScreenSignal {
    Restart,
    Quit,
    Wait
}

pub fn end_sceen(config: &Config, d: &mut RaylibDrawHandle, pos: (f32, f32)) -> EndScreenSignal {
    profile_function!("end_screen");
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