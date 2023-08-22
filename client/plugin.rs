use bevy::prelude::*;
use bevy_ggrs::GGRSSchedule;
use bevy_asset_loader::prelude::*;

use crate::components::MainCamera;
use crate::events::{TorpedoCollisionEvent, SpawnTorpedoEvent, FontResource};
use crate::enums::*;
use crate::resources::*;
use crate::constants::TEXT_FONT_SIZE;
use crate::systems::movement::*;
use crate::systems::menus::*;
use crate::systems::multiplayer::*;
use crate::systems::torpedos::*;
use crate::systems::utils::*;
use crate::systems::ui::{check_ui_interaction, UiHandling, handle_interaction_buttons};

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
            // Ship Building
            .add_system(setup_ship_builder.in_schedule(OnEnter(GameState::ShipBuilding)))
            .add_system(handle_ship_builder_buttons.run_if(in_state(GameState::ShipBuilding)))
            .add_system(do_ship_builder_parts.run_if(in_state(GameState::ShipBuilding)))
            .add_system(exit_ship_builder.in_schedule(OnExit(GameState::ShipBuilding)))
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
                spawn_mothership.after(setup_world)
            ).in_schedule(OnEnter(GameState::MatchMaking)));
    }
}



pub fn setup_world(mut commands: Commands, mut font_res: ResMut<FontResource>, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle{
        projection: OrthographicProjection { scaling_mode: bevy::render::camera::ScalingMode::FixedVertical(250.), ..default() },        
        ..default()
    }, MainCamera));
    commands.insert_resource(ClearColor {0: Color::BLACK});

    let font: Handle<Font> = asset_server.load("fonts/FallingSkyBlack.otf");

    let colors = Colors{ 
        normal_text: Color::rgb(0.8, 0.8, 0.8), 
        ship_text: Color::LIME_GREEN, 
        menu_background: Color::rgb(0.1, 0.1, 0.1), 
        node_background: Color::rgb(0.2, 0.2, 0.2),
        button_normal: Color::GRAY, 
        button_pressed: Color::DARK_GRAY 
    };
    
    font_res.font = font.clone();
    font_res.p1_font = TextStyle{ font: font.clone(), font_size: TEXT_FONT_SIZE, color: colors.ship_text };
    font_res.preview_font = TextStyle{ font: font.clone(), font_size: TEXT_FONT_SIZE, color: colors.ship_text.with_a(0.5) };
    
    commands.insert_resource(colors);
}

