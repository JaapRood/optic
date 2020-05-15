use cqrs_core::{Aggregate, AggregateEvent};

use crate::events::requests;
use crate::state::requests::RequestsState;

pub enum RequestsAggregate {
  Uninitialized,
  Created(RequestsState),
}

impl Default for RequestsAggregate {
  fn default() -> Self {
    RequestsAggregate::Uninitialized
  }
}

impl RequestsAggregate {
  pub fn get_state(&self) -> Option<&RequestsState> {
    match *self {
      RequestsAggregate::Uninitialized => None,
      RequestsAggregate::Created(ref state) => Some(state),
    }
  }
}

impl Aggregate for RequestsAggregate {
  #[inline(always)]
  fn aggregate_type() -> &'static str
  where
    Self: Sized,
  {
    "requests"
  }
}

impl AggregateEvent<RequestsAggregate> for requests::PathComponentAdded {
  fn apply_to(self, aggregate: &mut RequestsAggregate) {
    if let RequestsAggregate::Created(ref mut state) = aggregate {
      // implement and call state.with_path_component
    }
  }
}
