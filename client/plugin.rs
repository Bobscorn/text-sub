use bevy::prelude::*;
use bevy_ggrs::GGRSSchedule;
use bevy_asset_loader::prelude::*;

use crate::events::{TorpedoCollisionEvent, SpawnTorpedoEvent, FontResource};
use crate::game_state::GameState;
use crate::systems::*;
use crate::resources::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<GameState>()
            .add_loading_state(
                LoadingState::new(GameState::AssetLoading)
                    .continue_to_state(GameState::MatchMaking)
            )
            .add_collection_to_loading_state::<_, ImageAssets>(GameState::AssetLoading)
            .add_event::<TorpedoCollisionEvent>()
            .add_event::<SpawnTorpedoEvent>()
            .insert_resource(FontResource::default())
            .add_system(wait_for_players.run_if(in_state(GameState::MatchMaking)))
            .add_systems((
                fire_torpedo.after(player_action),
                player_action.before(move_projectile).before(accelerate_projectile),
                reload_torpedo.before(fire_torpedo),
                accelerate_projectile.before(move_projectile),
                move_projectile.after(fire_torpedo).after(player_action),
                process_torpedo_collision.after(move_projectile),
                do_torpedo_events.after(process_torpedo_collision),
                do_lifetime.after(do_torpedo_events)
            ).in_schedule(GGRSSchedule))
            .add_systems((
                setup_world.before(spawn_mothership), 
                start_matchbox_socket, 
                spawn_mothership
            ).in_schedule(OnEnter(GameState::MatchMaking)));
    }
}

