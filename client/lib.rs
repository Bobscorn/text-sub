mod start;
mod utils;
mod plugin;
mod events;
mod constants;
mod components;
mod macros;
mod resources;
mod implementations;
mod systems;
mod enums;
mod input;
mod world;
mod structs;
mod parts;

use start::start;
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn start_client() {
    set_panic_hook();
    start();
}
