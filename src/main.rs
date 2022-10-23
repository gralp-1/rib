pub mod tile;
pub mod world;
pub mod config;
pub mod player;
pub mod draw_sections;
mod update_sections;
use raylib::prelude::*;
use crate::draw_sections::{draw_tiles, draw_time, end_sceen};
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
    let mut timer_enabled = true;
    while !rl.window_should_close() {

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        world = control_update(world, &mut player, &d);

        draw_tiles(&mut world, &config, &mut d);


        draw_time(time, &config, &mut d);

        if timer_enabled {
            time += 1.0*d.get_frame_time();
        }
        
        // TODO(gralp): end screen restarting, quitting (use an enum)
        // TODO(gralp): fix positioning and colour and look into styling it
        // TODO(gralp): move whole time system into update_sections

        if time as i32 == config.default_time as i32 {
            timer_enabled = false;
            if end_sceen(&config, &mut d, (100.0, 100.0)) {
                timer_enabled = true;
                time = config.default_time;
            } else {
                std::process::exit(0);
            }
        }

        println!("{}", time);

    }
    // save world
    world.save_world();
}