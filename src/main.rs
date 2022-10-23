pub mod tile;
pub mod world;
pub mod config;
pub mod player;
pub mod draw_sections;
mod update_sections;
use raylib::prelude::*;
use raylib::rgui::RaylibDrawGui;
use crate::draw_sections::draw_tiles;
use crate::update_sections::control_update;
use crate::world::World;
use crate::config::Config;
use crate::player::Player;


fn main() {

    // get the position of the player and set it to an empty tile
    let config = Config::load_config();
    config.config_checks();

    let mut world = World::new(config.clone());

    let player_pos = world.player_pos;
    let mut player: Player = Player::new(player_pos);

    let (mut rl, thread) = raylib::init()
        .size(config.window_width, config.window_height)
        .title(&config.window_name)
        .vsync()
        .build();
    
    
    let mut time = 0.0;
    while !rl.window_should_close() {

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        world = control_update(world, &mut player, &d);

        // Draw the world as squares with a size of 32x32, with a 1px border and the colour of the tile in the world file
        draw_tiles(&mut world, &config, &mut d);
        if time as i32 == config.default_time as i32 {
            break;
        }
        if d.gui_button(rrect(0,0,50,50), Some(rstr!("hi"))) {
            println!("hi");
        }
        // d.draw_text(&format!("Time: {}", (config.default_time - time) as i32), 30, 30, 20, Color::WHITE);
        time += (1.0*d.get_frame_time())/1000.0;
        println!("{}", time/1000.0);

    }
    // save world
    world.save_world();
}