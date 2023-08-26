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

use wasm_bindgen::prelude::*;
use start::start;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn start_client() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once(); //enable halting the program upon a panic
    start();
}

pub fn main() {
    start_client();
}
