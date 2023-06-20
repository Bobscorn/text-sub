use bevy::prelude::*;

use crate::events::TorpedoCollisionEvent;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<TorpedoCollisionEvent>()
            .add_startup_system(hello_world);
    }
}

pub fn hello_world() {
    println!("Yo wassup world");
}

