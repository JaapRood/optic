#![allow(dead_code, unused_imports, unused_variables)]

use wasm_bindgen::prelude::*;

use optic_diff_engine::{SpecEvent, SpecProjection};

#[wasm_bindgen(start)]
pub fn init() {
  std::panic::set_hook(Box::new(console_error_panic_hook::hook));
  wasm_logger::init(wasm_logger::Config::default());
}

#[wasm_bindgen]
pub fn spec_from_events(spec_json: String) -> Result<WasmSpecProjection, JsValue> {
  let spec_events: Vec<SpecEvent> = serde_json::from_str(&spec_json).unwrap();
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
