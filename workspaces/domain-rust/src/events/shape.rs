use super::EventContext;

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
