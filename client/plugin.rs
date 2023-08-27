use bevy::prelude::*;
extern crate bevy_ggrs;

use bevy_ggrs::GgrsSchedule;
use bevy_asset_loader::prelude::*;

use crate::events::*;
use crate::enums::*;
use crate::resources::*;
use crate::systems::movement::*;
use crate::systems::menus::*;
use crate::systems::multiplayer::*;
use crate::systems::sync_ships::synchronise_peer_ships;
use crate::systems::torpedos::*;
use crate::systems::utils::*;
use crate::systems::ui::*;
use crate::world::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<GameState>()
            .add_loading_state(
                LoadingState::new(GameState::AssetLoading)
                    .continue_to_state(GameState::MainMenu)
            )
            .add_collection_to_loading_state::<_, ImageAssets>(GameState::AssetLoading)
            .add_event::<TorpedoCollisionEvent>()
            .add_event::<SpawnTorpedoEvent>()
            .insert_resource(FontResource::default())
            .insert_resource(UiHandling::default())
            // General Systems
            .add_systems(Startup, setup_world)
            .add_systems(Update, (check_ui_interaction,handle_interaction_buttons, handle_menu_buttons))
            // Main Menu
            .add_systems(OnEnter(GameState::MainMenu), setup_mainmenu.after(setup_world))
            .add_systems(OnExit(GameState::MainMenu), exit_main_menu)
            // sub Building
            .add_systems(OnEnter(GameState::SubBuilding), setup_sub_builder)
            .add_systems(Update, (sub_builder_part_buttons, do_sub_builder_parts, rotate_sub_preview).run_if(in_state(GameState::SubBuilding)))
            .add_systems(OnExit(GameState::SubBuilding), exit_sub_builder)
            // Match making
            .add_systems(Update, wait_for_players.run_if(in_state(GameState::MatchMaking)))
            // Sub synchronisation
            .add_systems(Update, synchronise_peer_ships.run_if(in_state(GameState::SubSyncing)))
            // In Game
            .add_systems(GgrsSchedule, (
                fire_torpedo.after(player_action),
                player_action.before(move_projectile).before(accelerate_projectile),
                reload_torpedo.before(fire_torpedo),
                accelerate_projectile.before(move_projectile),
                move_projectile.after(fire_torpedo).after(player_action),
                process_torpedo_collision.after(move_projectile),
                do_torpedo_events.after(process_torpedo_collision),
                do_lifetime.after(do_torpedo_events)
            ))
            .add_systems(OnEnter(GameState::MatchMaking), (
                start_matchbox_socket, 
                spawn_mothersub.after(setup_world)
            ));
    }
}
