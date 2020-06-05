use crate::state::requests::{HttpRequest, HttpResponse, RequestsState};

pub fn existing_requests(state: &RequestsState) -> impl Iterator<Item = &HttpRequest> {
  state.all_requests().filter(|request| !request.is_removed)
}

pub fn existing_responses(state: &RequestsState) -> impl Iterator<Item = &HttpResponse> {
  state
    .all_responses()
    .filter(|response| !response.is_removed)
}
