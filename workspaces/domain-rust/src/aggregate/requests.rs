use cqrs_core::{Aggregate, AggregateEvent, Event};

use crate::events::requests::RequestsEvent;
pub use crate::state::requests::RequestsState;

#[derive(Default)]
pub struct RequestsAggregate {
  pub state: RequestsState,
}

impl RequestsAggregate {
  pub fn get_state(&self) -> &RequestsState {
    &self.state
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

impl AggregateEvent<RequestsAggregate> for RequestsEvent {
  fn apply_to(self, aggregate: &mut RequestsAggregate) {
    let state = &mut aggregate.state;

    match self {
      // RequestsEvent::PathComponentAdded(evt) => {
      //   *state =
      // }
      _ => console_log!(
        "Missing application logic of '{}' event for '{}' aggregate",
        self.event_type(),
        RequestsAggregate::aggregate_type()
      ),
    }
  }
}
