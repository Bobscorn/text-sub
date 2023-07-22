use bevy::prelude::*;
use bevy_ggrs::GGRSPlugin;

use crate::components::BulletReady;
use crate::plugin::GamePlugin;
use crate::input::player_input;
use crate::systems::GgrsConfig;

pub fn start() {
    let mut app = App::new();

    GGRSPlugin::<GgrsConfig>::new()
        .with_input_system(player_input)
        .register_rollback_component::<Transform>()
        .register_rollback_component::<BulletReady>()
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
        .run();
}
