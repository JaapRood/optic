#![allow(dead_code)] // TODO: disallow dead code

use js_sys::Array;
use wasm_bindgen::prelude::*;

#[macro_use]
extern crate serde_derive;

#[macro_use]
mod macros;
mod aggregate;
mod events;
mod js;
mod queries;
mod state;

use aggregate::{Aggregate, OpticAggregate};
use events::OpticEvent;

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
pub fn rfc_aggregate_from_events(raw_events: &JsValue) -> Result<OpticAggregate, JsValue> {
    let events: Vec<OpticEvent> = raw_events.into_serde().unwrap();

    let mut aggregate = OpticAggregate::default();
    for event in events {
        aggregate.apply(event)
    }

    Ok(aggregate)
}

#[wasm_bindgen]
pub fn aggregate_endpoints(aggregate: &OpticAggregate) -> Result<Array, JsValue> {
    let state = aggregate.get_state();
    let endpoints = queries::endpoints(&state);

    console_log!("State: {:#?}", state);

    console_log!("Endpoints: {:#?}", endpoints);

    Ok(endpoints
        .iter()
        .map(js::endpoint_from)
        .map(JsValue::from)
        .collect())
}
