use bevy::prelude::*;
use bevy_pkv::*;

use crate::components::MainCamera;
use crate::events::*;
use crate::resources::*;
use crate::constants::*;

pub fn setup_world(mut commands: Commands,
    mut font_res: ResMut<FontResource>, 
    asset_server: Res<AssetServer>,
    storage: Res<PkvStore>
) {

    commands.spawn((Camera2dBundle{
        projection: OrthographicProjection { scaling_mode: bevy::render::camera::ScalingMode::FixedVertical(250.0), ..default() },        
        ..default()
    }, MainCamera));
    commands.insert_resource(ClearColor {0: Color::BLACK});

    let font: Handle<Font> = asset_server.load("fonts/FallingSkyBlack.otf");

    let colors = Colors{ 
        normal_text: Color::rgb(0.8, 0.8, 0.8), 
        submarine_text: Color::LIME_GREEN, 
        menu_background: Color::rgb(0.1, 0.1, 0.1), 
        node_background: Color::rgb(0.2, 0.2, 0.2),
        button_normal: Color::GRAY, 
        button_pressed: Color::DARK_GRAY 
    };
    
    font_res.font = font.clone();
    font_res.p1_font = TextStyle{ font: font.clone(), font_size: TEXT_FONT_SIZE, color: colors.submarine_text };
    font_res.preview_font = TextStyle{ font: font.clone(), font_size: TEXT_FONT_SIZE, color: colors.submarine_text.with_a(0.2) };
    
    commands.insert_resource(colors);

    //check cache for build
    let submarine = match storage.get::<Submarine>(CACHED_KEY) {
        Ok(item) => item,
        Err(_) => Submarine::default()
    };
    commands.insert_resource(submarine);
}
