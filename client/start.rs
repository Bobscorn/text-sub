use bevy::prelude::*;
use bevy_ggrs::GGRSPlugin;

use crate::plugin::GamePlugin;
use crate::systems::player_input;
use crate::systems::GgrsConfig;

pub fn start() {
    let mut app = App::new();

    GGRSPlugin::<GgrsConfig>::new()
        .with_input_system(player_input)
        .register_rollback_component::<Transform>()
        .build(&mut app);

    app.add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
}
