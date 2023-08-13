use bevy::prelude::*;
use bevy_ggrs::ggrs::PlayerHandle;

use crate::constants::MOTHERSHIP_SPEED;

pub const INPUT_FIRE: u8 =      1 << 0;
pub const INPUT_LEFT: u8 =      1 << 1;
pub const INPUT_RIGHT: u8 =     1 << 2;
pub const INPUT_FORWARD: u8 =   1 << 3;
pub const INPUT_BACKWARD: u8 =  1 << 4;
pub const INPUT_CCLOCKWISE: u8 = 1 << 5;
pub const INPUT_CLOCKWISE: u8 = 1 << 6; 

pub fn player_input(In(_handle): In<PlayerHandle>, keys: Res<Input<KeyCode>>) -> u8 {
    let mut input = 0u8;

    if keys.any_pressed([KeyCode::Up, KeyCode::W]) { //UP
        input |= INPUT_FORWARD;
    }

    if keys.any_pressed([KeyCode::Down, KeyCode::S]) { //DOWN
        input |= INPUT_BACKWARD;
    }

    if keys.any_pressed([KeyCode::Left, KeyCode::A]) { //LEFT
        input |= INPUT_LEFT;
    }

    if keys.any_pressed([KeyCode::Right, KeyCode::D]) { //RIGHT
        input |= INPUT_RIGHT;
    }

    if keys.any_pressed([KeyCode::Space, KeyCode::Z]) { //TORPEDO
        input |= INPUT_FIRE;
    }

    if keys.any_pressed([KeyCode::Q]) { // ROTATE COUNTER CLOCKWISE
        input |= INPUT_CCLOCKWISE;
    }

    if keys.any_pressed([KeyCode::E]) { // ROTATE CLOCKWISE
        input |= INPUT_CLOCKWISE;
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

pub fn get_rotation(input: u8) -> f32 {
    let mut angular_velocity = 0.0_f32;

    if input & INPUT_CCLOCKWISE != 0 {
        angular_velocity += -ROTATION_SPEED;
    }

    if input & INPUT_CLOCKWISE != 0 {
        angular_velocity += ROTATION_SPEED;
    }
    angular_velocity
}

pub fn fire(input: u8) -> bool {
    return input & INPUT_FIRE != 0;
}
