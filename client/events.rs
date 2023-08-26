use bevy::prelude::*;

#[derive(Debug, Event)]
pub struct TorpedoCollisionEvent {
    pub position: Vec3,
    pub damage: u8,
    pub radius: f32
}

#[derive(Debug, Event)]
pub struct SpawnTorpedoEvent {
    pub position: Vec3,
    pub velocity: Vec3
}
