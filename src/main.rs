mod components;
mod systems;
mod constants;
mod events;
mod plugin;

use bevy::prelude::*;
use components::*;
use events::TorpedoCollisionEvent;

fn main() {
    App::new()
        .add_event::<TorpedoCollisionEvent>()
        .add_plugins(DefaultPlugins)
        .add_plugin(crate::plugin::GamePlugin)
        .run();
}

