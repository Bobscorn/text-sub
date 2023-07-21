use bevy::prelude::*;
use bevy_ggrs::GGRSSchedule;

use crate::events::{TorpedoCollisionEvent, SpawnTorpedoEvent, FontResource};
use crate::game_state::GameState;
use crate::systems::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<GameState>()
            .add_event::<TorpedoCollisionEvent>()
            .add_event::<SpawnTorpedoEvent>()
            .insert_resource(FontResource::default())
            //.add_system(move_mothership)
            .add_system(spawn_torpedos)
            .add_systems((player_action.in_schedule(GGRSSchedule), wait_for_players))
            .add_startup_system(start_matchbox_socket)
            .add_startup_system(hello_world)
            .add_startup_system(spawn_mothership)
            .add_startup_system(setup_world.before(spawn_mothership));
    }
}

pub fn hello_world() {
    println!("Yo wassup world");
}

