
use crate::{world::World, player::Player};
use raylib::prelude::*;
use crate::player::Direction;

pub fn control_update(world: World, player: &mut Player, d: &RaylibDrawHandle) -> World {
    if d.is_key_pressed(KeyboardKey::KEY_W) || d.is_key_pressed(KeyboardKey::KEY_UP) {
        return player.move_player(Direction::Up, world);
    }
    if d.is_key_pressed(KeyboardKey::KEY_S) || d.is_key_pressed(KeyboardKey::KEY_DOWN) {
        return player.move_player(Direction::Down, world);
    }
    if d.is_key_pressed(KeyboardKey::KEY_A) || d.is_key_pressed(KeyboardKey::KEY_LEFT) {
        return player.move_player(Direction::Left, world);
    }
    if d.is_key_pressed(KeyboardKey::KEY_D) || d.is_key_pressed(KeyboardKey::KEY_RIGHT) {
        return player.move_player(Direction::Right, world);
    }
    world
}