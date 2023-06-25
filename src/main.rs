
mod components;
mod systems;
mod constants;
mod events;
mod plugin;

use bevy::prelude::*;

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once(); //enable halting the program upon a panic

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(crate::plugin::GamePlugin)
        .run();
}


