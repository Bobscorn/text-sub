use bevy::prelude::*;

use crate::events::{TorpedoCollisionEvent, SpawnTorpedoEvent};
use crate::systems::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<TorpedoCollisionEvent>()
            .add_event::<SpawnTorpedoEvent>()
            .add_system(move_mothership)
            .add_system(simple_input)
            .add_startup_system(hello_world)
            .add_startup_system(spawn_mothership)
            .add_startup_system(setup_world);
    }
}

pub fn hello_world() {
    println!("Yo wassup world");
}

