use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct RequestsState {
  path_components: HashMap<PathComponentId, PathComponent>,
  parent_path: HashMap<PathComponentId, PathComponentId>,
  request_parameters: HashMap<RequestParameterId, HttpRequestParameter>,
}

pub type PathComponentId = String;
pub type RequestId = String;
pub type RequestParameterId = String;
pub type ResponseId = String;
pub type ShapeId = String;

#[derive(Debug)]
struct PathComponent {
  path_id: PathComponentId,
  descriptor: PathComponentDescriptor,
  is_removed: bool,
}

#[derive(Debug)]
struct PathComponentDescriptor {
  parent_path_id: PathComponentId,
  name: String,
}

#[derive(Debug)]
struct HttpRequestParameter {
  parameter_id: RequestParameterId,
  request_paramater_descriptor: RequestParameterDescriptor,
  is_removed: bool,
}

#[derive(Debug)]
struct RequestParameterDescriptor {
  path_id: RequestId,
  http_method: String,
  location: String,
  name: String,
  shape_descriptor: RequestParameterShapeDescriptor, // bodyDescriptor: BodyDescriptor
}

#[derive(Debug)]
enum RequestParameterShapeDescriptor {
  Unset {},
  Shaped { shape_id: ShapeId, is_removed: bool },
}

impl RequestsState {
  pub fn with_path_component(
    &mut self,
    path_id: PathComponentId,
    parent_path_id: PathComponentId,
    name: String,
  ) {
    self.path_components.insert(
      path_id.clone(),
      PathComponent {
        path_id: path_id.clone(),
        descriptor: PathComponentDescriptor {
          parent_path_id,
          name,
        },
        is_removed: false,
      },
    );
  }

  pub fn with_request_parameter_by_path_and_method(
    &mut self,
    parameter_id: RequestParameterId,
    path_id: PathComponentId,
    http_method: String,
    parameter_location: String,
    name: String,
  ) {
    self.request_parameters.insert(
      parameter_id.clone(),
      HttpRequestParameter {
        parameter_id: parameter_id.clone(),
        request_paramater_descriptor: RequestParameterDescriptor {
          path_id,
          http_method,
          location: parameter_location,
          name,
          shape_descriptor: RequestParameterShapeDescriptor::Unset {},
        },
        is_removed: false,
      },
    );
  }
}
