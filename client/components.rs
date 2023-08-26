use bevy::prelude::*;
use crate::constants::{SUB_MAX_HEIGHT, SUB_MAX_WIDTH, EMPTY_CHAR};
use crate::enums::*;
use crate::resources::SubPiece;

#[derive(Component)]
pub struct MyButton {
    pub identifier: ButtonType
}

#[derive(Component)]
pub struct SubBuilderButton {
    pub part: SubPiece,
}

#[derive(Component, Reflect, Default)]
pub struct Velocity {
    pub value: Vec2
}

#[derive(Component, Reflect, Default)]
pub struct AngularVelocity {
    pub rotation: f32
}

#[derive(Component, Reflect, Default)]
pub struct Acceleration(pub Vec2);

#[derive(Component)]
pub struct Colour {
    pub hex_value: str
}

#[derive(Component)]
pub struct Character {
    pub characters_symbol: char
}

#[derive(Component)]
pub struct Structure {
    pub integrity: u8,
    pub max_integrity: u8
}

#[derive(Component)]
pub struct Armour {

}

#[derive(Component)]
pub struct Torpedo {
    pub damage: u8,
    pub detonate_radius: f32,
    pub explosion_radius: f32
}

impl Torpedo {
    pub fn default() -> Torpedo {
        Torpedo {
            damage: 10_u8,
            detonate_radius: 4.0_f32,
            explosion_radius: 7.5_f32
        }
    }
}

#[derive(Component, Reflect, Default)]
pub struct BulletReady(pub bool);

#[derive(Component)]
pub struct TorpedoLauncher {

}

#[derive(Component)]
pub struct Star {

}

#[derive(Component)]
pub struct Debris {

}

#[derive(Component)]
pub struct Shield {
    pub activated: bool
}

#[derive(Component)]
pub struct Reactor {

}

#[derive(Component)]
pub struct ReactorEnclosure {

}

#[derive(Component)]
pub struct Exhaust {

}

#[derive(Component)]
pub struct GatlingTurret {

}

#[derive(Component)]
pub struct Bullet {
    pub damage: u8
}

#[derive(Component)]
pub struct HangarDoor {

}

#[derive(Component)]
pub struct Propeller {

}

#[derive(Component)]
pub struct FighterDrone {

}

#[derive(Component)]
pub struct Mothersub {
    pub pieces: [[char; SUB_MAX_HEIGHT]; SUB_MAX_WIDTH],
    pub num_entities: u32
}

impl Default for Mothersub {
    fn default() -> Self {
        Mothersub { pieces: [[EMPTY_CHAR; SUB_MAX_HEIGHT]; SUB_MAX_WIDTH], num_entities: 0 }
    }
}

#[derive(Component)]
pub struct Player {
    pub handle: usize
}

#[derive(Component, Reflect, Default, Clone, Copy)]
pub struct Lifetime {
    pub lifetime: f32 // Remaining Lifetime in seconds
}

#[derive(Component)]
pub struct MainCamera;
