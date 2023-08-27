use bevy::prelude::*;
use bevy_ggrs::*;
use bevy_pkv::*;
use bevy_inspector_egui::quick::*;

use crate::components::*;
use crate::plugin::GamePlugin;
use crate::input::*;
use crate::implementations::*;
use crate::constants::*;

pub fn start() {
    let mut app = App::new();

    GgrsPlugin::<GgrsConfig>::new()
        .with_input_system(player_input)
        .register_rollback_component::<Transform>()
        .register_rollback_component::<BulletReady>()
        .register_rollback_component::<Velocity>()
        .register_rollback_component::<Acceleration>()
        .register_rollback_component::<Lifetime>()
        .build(&mut app);

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(GamePlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .insert_resource(PkvStore::new(TEAM_KEY, GAME_NAME))
        .run();
}
