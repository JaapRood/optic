use crate::aggregate::OpticState;
use crate::state::requests::PathComponentId;
use std::collections::HashSet;

mod requests;

#[derive(Debug)]
pub struct Endpoint<'a> {
  pub path_id: &'a PathComponentId,
  pub http_method: &'a String,
}

pub fn endpoints<'a>(state: &'a OpticState) -> Vec<Endpoint<'a>> {
  let requests = requests::existing_requests(state.requests);
  let responses = requests::existing_responses(state.requests);

  let request_path_method_pairs = requests.map(|request| {
    (
      &request.request_descriptor.http_method,
      &request.request_descriptor.path_component_id,
    )
  });

  let response_path_method_pairs = responses.map(|response| {
    (
      &response.response_descriptor.http_method,
      &response.response_descriptor.path_id,
    )
  });

  let path_method_pairs: HashSet<_> = request_path_method_pairs
    .chain(response_path_method_pairs)
    .collect();

  return path_method_pairs
    .iter()
    .map(|(http_method, path_id)| Endpoint {
      path_id,
      http_method,
    })
    .collect();
}
