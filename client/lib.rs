
use bevy::prelude::*;
use wasm_bindgen::prelude::*;

mod utils;
mod components;
mod systems;
mod constants;
mod events;
mod plugin;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, textship!");
}

#[wasm_bindgen]
pub fn start_client() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once(); //enable halting the program upon a panic

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(crate::plugin::GamePlugin)
        .run();
}
