#![allow(dead_code, unused_imports, unused_variables)]

use wasm_bindgen::prelude::*;

use optic_diff_engine::{SpecEvent, SpecProjection};

#[wasm_bindgen]
pub fn spec_from_events(raw_events: &JsValue) -> Result<(), JsValue> {
  Ok(())
}
