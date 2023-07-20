use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity {
    pub value: Vec2
}

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
pub struct Thruster {

}

#[derive(Component)]
pub struct FighterDrone {

}

#[derive(Component, Default)]
pub struct Mothership {
    pub num_entities: u32
}

#[derive(Component)]
pub struct Player;
