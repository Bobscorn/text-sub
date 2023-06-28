
mod start;
mod utils;
mod plugin;
mod systems;
mod events;
mod constants;
mod components;
use start::start;
use utils::set_panic_hook;

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
