use bevy::prelude::*;

pub fn start() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(crate::plugin::GamePlugin)
    .run();
}
