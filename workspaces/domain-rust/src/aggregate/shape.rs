use cqrs_core::{Aggregate, AggregateEvent, Event};

use crate::events::shape::ShapeEvent;

#[derive(Default, Debug)]
pub struct ShapeState {}

#[derive(Default)]
pub struct ShapeAggregate {
  pub state: ShapeState,
}

impl ShapeAggregate {
  pub fn get_state(&self) -> &ShapeState {
    &self.state
  }
}

impl Aggregate for ShapeAggregate {
  #[inline(always)]
  fn aggregate_type() -> &'static str
  where
    Self: Sized,
  {
    "shape"
  }
}

impl AggregateEvent<ShapeAggregate> for ShapeEvent {
  fn apply_to(self, aggregate: &mut ShapeAggregate) {
    let state = &mut aggregate.state;

    match self {
      _ => console_log!(
        "Missing application of '{}' event for '{}' aggregate",
        self.event_type(),
        ShapeAggregate::aggregate_type()
      ),
    }
  }
}
