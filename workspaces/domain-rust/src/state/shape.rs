use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct ShapeState {
  shapes: HashMap<ShapeId, ShapeEntity>,
}

pub type ShapeId = String;
pub type FieldId = String;
type ShapeParameterId = String;

#[derive(Debug)]
struct ShapeValue {
  is_user_defined: bool,
  base_shape_id: ShapeId,
  parameters: ShapeParametersDescriptor,
  field_ordering: Vec<FieldId>,
  name: String,
}

#[derive(Debug)]
struct ShapeEntity {
  shape_id: ShapeId,
  descriptor: ShapeValue,
  is_removed: bool,
}

#[derive(Debug, Deserialize)]
pub enum ShapeParametersDescriptor {
  NoParameterList,
  StaticParameterList(StaticShapeParametersDescriptor),
  DynamicParameterList(DynamicShapeParametersDescriptor),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StaticShapeParametersDescriptor {
  shape_parameter_ids: Vec<ShapeParameterId>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DynamicShapeParametersDescriptor {
  shape_parameter_ids: Vec<ShapeParameterId>,
}

impl ShapeState {
  pub fn with_shape(
    &mut self,
    shape_id: ShapeId,
    assigned_shape_id: ShapeId,
    parameters: ShapeParametersDescriptor,
    name: String,
  ) {
    self.shapes.insert(
      shape_id.clone(),
      ShapeEntity {
        shape_id: shape_id.clone(),
        descriptor: ShapeValue {
          is_user_defined: true,
          base_shape_id: assigned_shape_id,
          parameters,
          name,
          field_ordering: vec![],
        },
        is_removed: false,
      },
    );
  }
}
