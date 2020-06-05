use crate::state::requests::PathComponentId;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::queries;

#[wasm_bindgen]
pub struct Endpoint {
  path_id: PathComponentId,
  http_method: String,
}

pub fn endpoint_from(endpoint: &queries::Endpoint) -> Endpoint {
  Endpoint {
    path_id: (*endpoint.path_id).to_string(),
    http_method: (*endpoint.http_method).to_string(),
  }
}

#[wasm_bindgen]
impl Endpoint {
  #[wasm_bindgen(getter)]
  pub fn path_id(&self) -> PathComponentId {
    self.path_id.clone()
  }

  #[wasm_bindgen(getter)]
  pub fn http_method(&self) -> String {
    self.http_method.clone()
  }
}
