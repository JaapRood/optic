pub use cqrs_core::Aggregate;
use cqrs_core::AggregateEvent;

use crate::events::OpticEvent;

pub mod requests;
pub mod rfc;
pub mod shape;

#[derive(Default)]
pub struct OpticAggregate {
  requests: requests::RequestsAggregate,
  rfc: rfc::RfcAggregate,
  shape: shape::ShapeAggregate,
}

impl Aggregate for OpticAggregate {
  #[inline(always)]
  fn aggregate_type() -> &'static str
  where
    Self: Sized,
  {
    "optic"
  }
}

impl AggregateEvent<OpticAggregate> for OpticEvent {
  fn apply_to(self, aggregate: &mut OpticAggregate) {
    match self {
      OpticEvent::RequestsEvent(evt) => aggregate.requests.apply(evt),
      OpticEvent::RfcEvent(evt) => aggregate.rfc.apply(evt),
      OpticEvent::ShapeEvent(evt) => aggregate.shape.apply(evt),
    }
  }
}
