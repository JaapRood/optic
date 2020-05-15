use super::EventContext;
use cqrs_core::Event;

type ShapeId = String;
type ShapeParameterId = String;
type FieldId = String;

#[derive(Deserialize)]
pub enum ShapeEvent {
  ShapeAdded(ShapeAdded),
  BaseShapeSet(BaseShapeSet),
  ShapeRenamed(ShapeRenamed),
  ShapeRemoved(ShapeRemoved),
  ShapeParameterAdded(ShapeParameterAdded),
  ShapeParameterShapeSet(ShapeParameterShapeSet),
  ShapeParameterRenamed(ShapeParameterRenamed),
  ShapeParameterRemoved(ShapeParameterRemoved),

  FieldAdded(FieldAdded),
  FieldShapeSet(FieldShapeSet),
  FieldRenamed(FieldRenamed),
  FieldRemoved(FieldRemoved),
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShapeAdded {
  shape_id: ShapeId,
  base_shape_id: ShapeId,
  // parameters: ShapeParametersDescriptor,
  name: String,
  event_context: Option<EventContext>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseShapeSet {
  shape_id: ShapeId,
  base_shape_id: ShapeId,
  event_context: Option<EventContext>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShapeRenamed {
  shape_id: ShapeId,
  name: String,
  event_context: Option<EventContext>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShapeRemoved {
  shape_id: ShapeId,
  event_context: Option<EventContext>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShapeParameterAdded {
  shape_parameter_id: ShapeParameterId,
  shape_id: ShapeId,
  name: String,
  // shapeDescriptor: ParameterShapeDescriptor,
  event_context: Option<EventContext>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShapeParameterShapeSet {
  // shapeDescriptor: ParameterShapeDescriptor,
  event_context: Option<EventContext>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShapeParameterRenamed {
  shape_parameter_id: ShapeParameterId,
  name: String,
  event_context: Option<EventContext>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShapeParameterRemoved {
  shape_parameter_id: ShapeParameterId,
  event_context: Option<EventContext>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldAdded {
  field_id: FieldId,
  shape_id: ShapeId,
  name: String,
  // shapeDescriptor: FieldShapeDescriptor,
  event_context: Option<EventContext>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldShapeSet {
  // shapeDescriptor: FieldShapeDescriptor,
  event_context: Option<EventContext>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldRenamed {
  field_id: FieldId,
  name: String,
  event_context: Option<EventContext>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldRemoved {
  field_id: FieldId,
  event_context: Option<EventContext>,
}

impl Event for ShapeAdded {
  fn event_type(&self) -> &'static str {
    "ShapeAdded"
  }
}

impl Event for BaseShapeSet {
  fn event_type(&self) -> &'static str {
    "BaseShapeSet"
  }
}

impl Event for ShapeRenamed {
  fn event_type(&self) -> &'static str {
    "ShapeRenamed"
  }
}

impl Event for ShapeRemoved {
  fn event_type(&self) -> &'static str {
    "ShapeRemoved"
  }
}

impl Event for ShapeParameterAdded {
  fn event_type(&self) -> &'static str {
    "ShapeParameterAdded"
  }
}

impl Event for ShapeParameterShapeSet {
  fn event_type(&self) -> &'static str {
    "ShapeParameterShapeSet"
  }
}

impl Event for ShapeParameterRenamed {
  fn event_type(&self) -> &'static str {
    "ShapeParameterRenamed"
  }
}

impl Event for ShapeParameterRemoved {
  fn event_type(&self) -> &'static str {
    "ShapeParameterRemoved"
  }
}

impl Event for FieldAdded {
  fn event_type(&self) -> &'static str {
    "FieldAdded"
  }
}

impl Event for FieldShapeSet {
  fn event_type(&self) -> &'static str {
    "FieldShapeSet"
  }
}

impl Event for FieldRenamed {
  fn event_type(&self) -> &'static str {
    "FieldRenamed"
  }
}

impl Event for FieldRemoved {
  fn event_type(&self) -> &'static str {
    "FieldRemoved"
  }
}
