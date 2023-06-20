
mod components;
mod systems;
mod constants;
mod events;
mod plugin;


use bevy::prelude::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(crate::plugin::GamePlugin)
        .run();
}


