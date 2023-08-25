use bevy::prelude::*;
use bevy_ggrs::GGRSSchedule;
use bevy_asset_loader::prelude::*;

use crate::events::*;
use crate::enums::*;
use crate::resources::*;
use crate::systems::movement::*;
use crate::systems::menus::*;
use crate::systems::multiplayer::*;
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
            .add_startup_system(setup_world)
            .add_system(check_ui_interaction)
            .add_system(handle_interaction_buttons)
            .add_system(handle_menu_buttons)
            // Main Menu
            .add_system(setup_mainmenu.after(setup_world).in_schedule(OnEnter(GameState::MainMenu)))
            .add_system(exit_main_menu.in_schedule(OnExit(GameState::MainMenu)))
            // sub Building
            .add_system(setup_sub_builder.in_schedule(OnEnter(GameState::SubBuilding)))
            .add_system(sub_builder_piece_buttons.run_if(in_state(GameState::SubBuilding)))
            .add_system(do_sub_builder_parts.run_if(in_state(GameState::SubBuilding)))
            .add_system(exit_sub_builder.in_schedule(OnExit(GameState::SubBuilding)))
            // Match making
            .add_system(wait_for_players.run_if(in_state(GameState::MatchMaking)))
            // In Game
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
                start_matchbox_socket, 
                spawn_mothersub.after(setup_world)
            ).in_schedule(OnEnter(GameState::MatchMaking)));
    }
}
