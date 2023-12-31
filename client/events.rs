use bevy::prelude::*;

#[derive(Debug)]
pub struct TorpedoCollisionEvent {
    pub position: Vec3,
    pub damage: u8,
    pub radius: f32
}

#[derive(Debug)]
pub struct SpawnTorpedoEvent {
    pub position: Vec3,
    pub velocity: Vec3
}

#[derive(Resource, Default)]
pub struct FontResource {
    pub font: TextStyle,
    pub p1_font: TextStyle,
}
