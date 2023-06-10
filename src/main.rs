mod components;
mod systems;
mod constants;

use bevy::prelude::*;
use components::*;

fn main() {
    App::new()
        .add_system(hello_world)
        .run();
}

fn hello_world() {
    println!("Hello world!");
}
