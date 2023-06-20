use bevy::prelude::*;

use crate::events::TorpedoCollisionEvent;
use crate::systems::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<TorpedoCollisionEvent>()
            .add_system(print_position_system)
            .add_startup_system(hello_world)
            .add_startup_system(setup_world);
    }
}

pub fn hello_world() {
    println!("Yo wassup world");
}

