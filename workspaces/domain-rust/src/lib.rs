use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[macro_use]
extern crate serde_derive;

mod events;
use events::Event;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    Ok(())
}

#[wasm_bindgen]
pub fn rfc_state_from_events(raw_events: &JsValue) -> Result<(), JsValue> {
    console::log_1(&raw_events);

    let events: Vec<HashMap<String, Event>> = raw_events.into_serde().unwrap();

    Ok(())
}
