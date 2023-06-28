use bevy::prelude::*;
use crate::plugin::GamePlugin;

pub fn start() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(GamePlugin)
    .run();
}
