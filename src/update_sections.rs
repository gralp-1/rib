use std::sync::Arc;

use crate::{world::World, player::Player};
use raylib::prelude::*;
use crate::player::Direction;

pub fn control_update(world: World, player: &mut Player, d: &RaylibDrawHandle) -> World {
    if d.is_key_pressed(KeyboardKey::KEY_W) {
        return player.move_player(Direction::Up, world);
    }
    if d.is_key_pressed(KeyboardKey::KEY_S) {
        return player.move_player(Direction::Down, world);
    }
    if d.is_key_pressed(KeyboardKey::KEY_A) {
        return player.move_player(Direction::Left, world);
    }
    if d.is_key_pressed(KeyboardKey::KEY_D) {
        return player.move_player(Direction::Right, world);
    }
    world
}