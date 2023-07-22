use bevy::prelude::*;
use bevy_ggrs::ggrs::PlayerHandle;

pub const INPUT_FIRE: u8 =      1 << 0;
pub const INPUT_LEFT: u8 =      1 << 1;
pub const INPUT_RIGHT: u8 =     1 << 2;
pub const INPUT_FORWARD: u8 =   1 << 3;
pub const INPUT_BACKWARD: u8 =  1 << 4;

pub fn player_input(In(_handle): In<PlayerHandle>, keys: Res<Input<KeyCode>>) -> u8 {
    let mut input = 0u8;

    if keys.any_pressed([KeyCode::Up, KeyCode::W]) {
        input |= INPUT_FORWARD;
    }

    if keys.any_pressed([KeyCode::Down, KeyCode::S]) {
        input |= INPUT_BACKWARD;
    }

    if keys.any_pressed([KeyCode::Left, KeyCode::A]) {
        input |= INPUT_LEFT;
    }

    if keys.any_pressed([KeyCode::Right, KeyCode::D]) {
        input |= INPUT_RIGHT;
    }

    if keys.any_pressed([KeyCode::Space, KeyCode::Z]) {
        input |= INPUT_FIRE;
    }

    input
}

pub fn direction(input: u8) -> Vec2 {
    let mut direction = Vec2::ZERO;
    if input & INPUT_FORWARD != 0 {
        direction.y += 1.;
    }
    if input & INPUT_BACKWARD != 0 {
        direction.y -= 1.;
    }
    if input & INPUT_RIGHT != 0 {
        direction.x += 1.;
    }
    if input & INPUT_LEFT != 0 {
        direction.x -= 1.;
    }
    direction
}

pub fn fire(input: u8) -> bool {
    return input & INPUT_FIRE != 0;
}
