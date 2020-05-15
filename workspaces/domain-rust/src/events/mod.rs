#![allow(dead_code)]

mod requests;
mod rfc;
mod shape;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct EventContext {
  client_id: String,
  client_session_id: String,
  client_command_batch_id: String,
  created_at: String,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Event {
  RequestsEvent(requests::RequestsEvent),
  RfcEvent(rfc::RfcEvent),
  ShapeEvent(shape::ShapeEvent),
}
