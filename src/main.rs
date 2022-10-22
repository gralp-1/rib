pub mod tile;
pub mod world;
use raylib::prelude::*;
use crate::world::World;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Hello, World")
        .build();
    // Load world from ./world.txt
    // print current working directory
    let world = World::new("./world.txt");
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}