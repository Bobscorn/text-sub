mod components;
mod systems;
mod constants;
mod events;

use bevy::prelude::*;
use components::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(hello_world)
        .run();
}

fn hello_world() {
    println!("Hello world!");
}
