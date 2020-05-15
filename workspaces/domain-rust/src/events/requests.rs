use super::EventContext;

type PathComponentId = String;
type RequestId = String;
type RequestParameterId = String;
type ResponseId = String;

#[derive(Deserialize)]
pub enum RequestsEvent {
  // path components
  PathComponentAdded(PathComponentAdded),
  PathComponentRenamed(PathComponentRenamed),
  PathComponentRemoved(PathComponentRemoved),

  // path parameters
  PathParameterAdded(PathParameterAdded),
  PathParameterRenamed(PathParameterRenamed),
  PathParameterRemoved(PathParameterRemoved),

  // request parameters
  RequestParameterAddedByPathAndMethod(RequestParameterAddedByPathAndMethod),
  RequestParameterRenamed(RequestParameterRenamed),
  RequestParameterShapeSet(RequestParameterShapeSet),
  RequestParameterShapeUnset(RequestParameterShapeUnset),
  RequestParameterRemoved(RequestParameterRemoved),

  // Request events
  RequestAdded(RequestAdded),
  RequestContentTypeSet(RequestContentTypeSet),
  RequestBodySet(RequestBodySet),
  RequestBodyUnset(RequestBodyUnset),

  // Response events
  ResponseAddedByPathAndMethod(ResponseAddedByPathAndMethod),
  ResponseStatusCodeSet(ResponseStatusCodeSet),
  ResponseContentTypeSet(ResponseContentTypeSet),
  ResponseBodySet(ResponseBodySet),
  ResponseBodyUnset(ResponseBodyUnset),
  ResponseRemoved(ResponseRemoved),
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PathComponentAdded {
  path_id: PathComponentId,
  parent_path_id: PathComponentId,
  name: String,
  event_context: Option<EventContext>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PathComponentRenamed {
  path_id: PathComponentId,
  name: String,
  event_context: Option<EventContext>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PathComponentRemoved {
  path_id: PathComponentId,
  event_context: Option<EventContext>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PathParameterAdded {
  path_id: PathComponentId,
  parent_path_id: PathComponentId,
  name: String,
  event_context: Option<EventContext>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PathParameterRenamed {
  path_id: PathComponentId,
  name: String,
  event_context: Option<EventContext>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PathParameterRemoved {
  path_id: PathComponentId,
  name: String,
  event_context: Option<EventContext>,
}

#[derive(Deserialize)] // request parameters
#[serde(rename_all = "camelCase")]
pub struct RequestParameterAddedByPathAndMethod {
  parameter_id: RequestParameterId,
  path_id: PathComponentId,
  http_method: String,
  parameter_location: String,
  name: String,
  event_context: Option<EventContext>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestParameterRenamed {
  parameter_id: RequestParameterId,
  name: String,
  event_context: Option<EventContext>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestParameterShapeSet {
  parameter_id: RequestParameterId,
  // parameterDescriptor: ShapedRequestParameterShapeDescriptor,
  event_context: Option<EventContext>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestParameterShapeUnset {
  parameter_id: RequestParameterId,
  event_context: Option<EventContext>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestParameterRemoved {
  parameter_id: RequestParameterId,
  event_context: Option<EventContext>,
}

#[derive(Deserialize)] // Request events
#[serde(rename_all = "camelCase")]
pub struct RequestAdded {
  request_id: RequestId,
  path_id: PathComponentId,
  http_method: String,
  event_context: Option<EventContext>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestContentTypeSet {
  request_id: RequestId,
  http_content_type: String,
  event_context: Option<EventContext>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestBodySet {
  request_id: RequestId,
  // bodyDescriptor: ShapedBodyDescriptor,
  event_context: Option<EventContext>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestBodyUnset {
  request_id: RequestId,
  event_context: Option<EventContext>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestRemoved {
  request_id: RequestId,
  event_context: Option<EventContext>,
}

#[derive(Deserialize)] // Response events
#[serde(rename_all = "camelCase")]
pub struct ResponseAddedByPathAndMethod {
  response_id: ResponseId,
  path_id: PathComponentId,
  http_method: String,
  http_status_code: u16,
  event_context: Option<EventContext>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseStatusCodeSet {
  response_id: ResponseId,
  http_status_code: u16,
  event_context: Option<EventContext>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseContentTypeSet {
  response_id: ResponseId,
  http_content_type: String,
  event_context: Option<EventContext>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseBodySet {
  response_id: ResponseId,
  // bodyDescriptor: ShapedBodyDescriptor,
  event_context: Option<EventContext>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseBodyUnset {
  response_id: ResponseId,
  event_context: Option<EventContext>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseRemoved {
  response_id: ResponseId,
  event_context: Option<EventContext>,
}
