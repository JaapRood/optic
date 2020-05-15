#![allow(dead_code)]

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
  RequestsEvent(RequestsEvent),
  RfcEvent(RfcEvent),
  ShapeEvent(ShapeEvent),
}

// RFC Events
// -----------
#[derive(Deserialize)]
enum RfcEvent {
  #[serde(rename_all = "camelCase")]
  ContributionAdded {
    id: String,
    key: String,
    value: String,
    event_context: Option<EventContext>,
  },
  #[serde(rename_all = "camelCase")]
  APINamed {
    name: String,
    event_context: Option<EventContext>,
  },

  #[serde(rename_all = "camelCase")]
  GitStateSet {
    branch_name: String,
    commit_id: String,
    event_context: Option<EventContext>,
  },

  #[serde(rename_all = "camelCase")]
  BatchCommitStarted {
    batch_id: String,
    commit_message: String,
    event_context: Option<EventContext>,
  },

  #[serde(rename_all = "camelCase")]
  BatchCommitEnded {
    batch_id: String,
    event_context: Option<EventContext>,
  },
}

// Requests events
// ---------------

type PathComponentId = String;
type RequestId = String;
type RequestParameterId = String;
type ResponseId = String;

#[derive(Deserialize)]
enum RequestsEvent {
  // path components
  #[serde(rename_all = "camelCase")]
  PathComponentAdded {
    path_id: PathComponentId,
    parent_path_id: PathComponentId,
    name: String,
    event_context: Option<EventContext>,
  },
  #[serde(rename_all = "camelCase")]
  PathComponentRenamed {
    path_id: PathComponentId,
    name: String,
    event_context: Option<EventContext>,
  },
  #[serde(rename_all = "camelCase")]
  PathComponentRemoved {
    path_id: PathComponentId,
    event_context: Option<EventContext>,
  },

  #[serde(rename_all = "camelCase")]
  // path parameters
  PathParameterAdded {
    path_id: PathComponentId,
    parent_path_id: PathComponentId,
    name: String,
    event_context: Option<EventContext>,
  },
  #[serde(rename_all = "camelCase")]
  // TODO: add PathParameterShapeSet
  PathParameterRenamed {
    path_id: PathComponentId,
    name: String,
    event_context: Option<EventContext>,
  },
  #[serde(rename_all = "camelCase")]
  PathParameterRemoved {
    path_id: PathComponentId,
    name: String,
    event_context: Option<EventContext>,
  },

  // request parameters
  #[serde(rename_all = "camelCase")]
  RequestParameterAddedByPathAndMethod {
    parameter_id: RequestParameterId,
    path_id: PathComponentId,
    http_method: String,
    parameter_location: String,
    name: String,
    event_context: Option<EventContext>,
  },
  #[serde(rename_all = "camelCase")]
  RequestParameterRenamed {
    parameter_id: RequestParameterId,
    name: String,
    event_context: Option<EventContext>,
  },
  #[serde(rename_all = "camelCase")]
  RequestParameterShapeSet {
    parameter_id: RequestParameterId,
    // parameterDescriptor: ShapedRequestParameterShapeDescriptor,
    event_context: Option<EventContext>,
  },
  #[serde(rename_all = "camelCase")]
  RequestParameterShapeUnset {
    parameter_id: RequestParameterId,
    event_context: Option<EventContext>,
  },
  #[serde(rename_all = "camelCase")]
  RequestParameterRemoved {
    parameter_id: RequestParameterId,
    event_context: Option<EventContext>,
  },

  // Request events
  #[serde(rename_all = "camelCase")]
  RequestAdded {
    request_id: RequestId,
    path_id: PathComponentId,
    http_method: String,
    event_context: Option<EventContext>,
  },
  #[serde(rename_all = "camelCase")]
  RequestContentTypeSet {
    request_id: RequestId,
    http_content_type: String,
    event_context: Option<EventContext>,
  },
  #[serde(rename_all = "camelCase")]
  RequestBodySet {
    request_id: RequestId,
    // bodyDescriptor: ShapedBodyDescriptor,
    event_context: Option<EventContext>,
  },
  #[serde(rename_all = "camelCase")]
  RequestBodyUnset {
    request_id: RequestId,
    event_context: Option<EventContext>,
  },
  #[serde(rename_all = "camelCase")]
  RequestRemoved {
    request_id: RequestId,
    event_context: Option<EventContext>,
  },

  // Response events
  #[serde(rename_all = "camelCase")]
  ResponseAddedByPathAndMethod {
    response_id: ResponseId,
    path_id: PathComponentId,
    http_method: String,
    http_status_code: u16,
    event_context: Option<EventContext>,
  },
  #[serde(rename_all = "camelCase")]
  ResponseStatusCodeSet {
    response_id: ResponseId,
    http_status_code: u16,
    event_context: Option<EventContext>,
  },

  #[serde(rename_all = "camelCase")]
  ResponseContentTypeSet {
    response_id: ResponseId,
    http_content_type: String,
    event_context: Option<EventContext>,
  },
  #[serde(rename_all = "camelCase")]
  ResponseBodySet {
    response_id: ResponseId,
    // bodyDescriptor: ShapedBodyDescriptor,
    event_context: Option<EventContext>,
  },
  #[serde(rename_all = "camelCase")]
  ResponseBodyUnset {
    response_id: ResponseId,
    event_context: Option<EventContext>,
  },
  #[serde(rename_all = "camelCase")]
  ResponseRemoved {
    response_id: ResponseId,
    event_context: Option<EventContext>,
  },
}

// Shape events
// ------------

type ShapeId = String;
type ShapeParameterId = String;
type FieldId = String;

#[derive(Deserialize)]
enum ShapeEvent {
  #[serde(rename_all = "camelCase")]
  ShapeAdded {
    shape_id: ShapeId,
    base_shape_id: ShapeId,
    // parameters: ShapeParametersDescriptor,
    name: String,
    event_context: Option<EventContext>,
  },
  #[serde(rename_all = "camelCase")]
  BaseShapeSet {
    shape_id: ShapeId,
    base_shape_id: ShapeId,
    event_context: Option<EventContext>,
  },
  #[serde(rename_all = "camelCase")]
  ShapeRenamed {
    shape_id: ShapeId,
    name: String,
    event_context: Option<EventContext>,
  },
  #[serde(rename_all = "camelCase")]
  ShapeRemoved {
    shape_id: ShapeId,
    event_context: Option<EventContext>,
  },

  #[serde(rename_all = "camelCase")]
  ShapeParameterAdded {
    shape_parameter_id: ShapeParameterId,
    shape_id: ShapeId,
    name: String,
    // shapeDescriptor: ParameterShapeDescriptor,
    event_context: Option<EventContext>,
  },
  #[serde(rename_all = "camelCase")]
  ShapeParameterShapeSet {
    // shapeDescriptor: ParameterShapeDescriptor,
    event_context: Option<EventContext>,
  },
  #[serde(rename_all = "camelCase")]
  ShapeParameterRenamed {
    shape_parameter_id: ShapeParameterId,
    name: String,
    event_context: Option<EventContext>,
  },
  #[serde(rename_all = "camelCase")]
  ShapeParameterRemoved {
    shape_parameter_id: ShapeParameterId,
    event_context: Option<EventContext>,
  },

  #[serde(rename_all = "camelCase")]
  FieldAdded {
    field_id: FieldId,
    shape_id: ShapeId,
    name: String,
    // shapeDescriptor: FieldShapeDescriptor,
    event_context: Option<EventContext>,
  },
  #[serde(rename_all = "camelCase")]
  FieldShapeSet {
    // shapeDescriptor: FieldShapeDescriptor,
    event_context: Option<EventContext>,
  },
  #[serde(rename_all = "camelCase")]
  FieldRenamed {
    field_id: FieldId,
    name: String,
    event_context: Option<EventContext>,
  },
  #[serde(rename_all = "camelCase")]
  FieldRemoved {
    field_id: FieldId,
    event_context: Option<EventContext>,
  },
}
