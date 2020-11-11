#![allow(dead_code, unused_imports, unused_variables)]

use wasm_bindgen::prelude::*;

use optic_diff_engine::{SpecEvent, SpecProjection};
use serde_wasm_bindgen;

#[wasm_bindgen]
pub fn spec_from_events(raw_events: JsValue) -> Result<WasmSpecProjection, JsValue> {
  let spec_events: Vec<SpecEvent> = serde_wasm_bindgen::from_value(raw_events)?;
  let spec_projection = SpecProjection::from(spec_events);

  Ok(WasmSpecProjection::from(spec_projection))
}

#[wasm_bindgen]
pub struct WasmSpecProjection {
  projection: SpecProjection,
}

impl From<SpecProjection> for WasmSpecProjection {
  fn from(projection: SpecProjection) -> Self {
    Self { projection }
  }
}
