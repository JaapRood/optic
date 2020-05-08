#[derive(Deserialize)]
struct EventContext {
  clientId: String,
  clientSessionId: String,
  clientCommandBatchId: String,
  createdAt: String,
}

#[derive(Deserialize)]
pub enum Event {
  RfcEvent,
  RequestsEvent,
}

// RFC Events
// -----------
#[derive(Deserialize)]
enum RfcEvent {
  ContributionAdded,
  APINamed,
  GitStateSet,
  BatchCommitStarted,
  BatchCommitEnded,
}

#[derive(Deserialize)]
struct ContributionAdded {
  id: String,
  key: String,
  value: String,
  eventContext: Option<EventContext>,
}

#[derive(Deserialize)]
struct APINamed {
  name: String,
  eventcontext: Option<EventContext>,
}

#[derive(Deserialize)]
struct GitStateSet {
  branchName: String,
  commitId: String,
  eventContext: Option<EventContext>,
}

#[derive(Deserialize)]
struct BatchCommitStarted {
  batchId: String,
  commitMessage: String,
  eventContext: Option<EventContext>,
}

#[derive(Deserialize)]
struct BatchCommitEnded {
  batchId: String,
  eventContext: Option<EventContext>,
}

// Requests events
// ---------------

type PathComponentId = String;
type RequestId = String;
type RequestParameterId = String;
type ResponseId = String;

enum RequestsEvent {
  PathComponentAdded,
  PathComponentRenamed,
  PathComponentRemoved,
  PathParameterAdded,
  PathParameterShapeSet,
  PathParameterRenamed,
  PathParameterRemoved,
  RequestParameterAddedByPathAndMethod,
  RequestParameterRenamed,
  RequestParameterShapeSet,
  RequestParameterShapeUnset,
  RequestParameterRemoved,
  RequestAdded,
  RequestContentTypeSet,
  RequestBodySet,
  RequestBodyUnset,
  RequestRemoved,
  ResponseAddedByPathAndMethod,
  ResponseStatusCodeSet,
  ResponseContentTypeSet,
  ResponseBodySet,
  ResponseBodyUnset,
  ResponseRemoved,
}

// path components
#[derive(Deserialize)]
struct PathComponentAdded {
  pathId: PathComponentId,
  parentPathId: PathComponentId,
  name: String,
  eventContext: Option<EventContext>,
}
#[derive(Deserialize)]
struct PathComponentRenamed {
  pathId: PathComponentId,
  name: String,
  eventContext: Option<EventContext>,
}
#[derive(Deserialize)]
struct PathComponentRemoved {
  pathId: PathComponentId,
  eventContext: Option<EventContext>,
}

// path parameters
#[derive(Deserialize)]
struct PathParameterAdded {
  pathId: PathComponentId,
  parentPathId: PathComponentId,
  name: String,
  eventContext: Option<EventContext>,
}
// TODO: add PathParameterShapeSet
#[derive(Deserialize)]
struct PathParameterRenamed {
  pathId: PathComponentId,
  name: String,
  eventContext: Option<EventContext>,
}
#[derive(Deserialize)]
struct PathParameterRemoved {
  pathId: PathComponentId,
  name: String,
  eventContext: Option<EventContext>,
}

// request parameters
#[derive(Deserialize)]
struct RequestParameterAddedByPathAndMethod {
  parameterId: RequestParameterId,
  pathId: PathComponentId,
  httpMethod: String,
  parameterLocation: String,
  name: String,
  eventContext: Option<EventContext>,
}
#[derive(Deserialize)]
struct RequestParameterRenamed {
  parameterId: RequestParameterId,
  name: String,
  eventContext: Option<EventContext>,
}
// #[derive(Deserialize)]
// struct RequestParameterShapeSet {
//   parameterId: RequestParameterId,
//   parameterDescriptor: ShapedRequestParameterShapeDescriptor,
//   eventContext: Option<EventContext>,
// }
#[derive(Deserialize)]
struct RequestParameterShapeUnset {
  parameterId: RequestParameterId,
  eventContext: Option<EventContext>,
}
#[derive(Deserialize)]
struct RequestParameterRemoved {
  parameterId: RequestParameterId,
  eventContext: Option<EventContext>,
}

// Request events
#[derive(Deserialize)]
struct RequestAdded {
  requestId: RequestId,
  pathId: PathComponentId,
  httpMethod: String,
  eventContext: Option<EventContext>,
}
#[derive(Deserialize)]
struct RequestContentTypeSet {
  requestId: RequestId,
  httpContentType: String,
  eventContext: Option<EventContext>,
}
#[derive(Deserialize)]
struct RequestBodySet {
  requestId: RequestId,
  // bodyDescriptor: ShapedBodyDescriptor,
  eventContext: Option<EventContext>,
}
#[derive(Deserialize)]
struct RequestBodyUnset {
  requestId: RequestId,
  eventContext: Option<EventContext>,
}
#[derive(Deserialize)]
struct RequestRemoved {
  requestId: RequestId,
  eventContext: Option<EventContext>,
}

// Response events
#[derive(Deserialize)]
struct ResponseAddedByPathAndMethod {
  responseId: ResponseId,
  pathId: PathComponentId,
  httpMethod: String,
  httpStatusCode: u8,
  eventContext: Option<EventContext>,
}
#[derive(Deserialize)]
struct ResponseStatusCodeSet {
  responseId: ResponseId,
  httpStatusCode: u8,
  eventContext: Option<EventContext>,
}
#[derive(Deserialize)]
struct ResponseContentTypeSet {
  responseId: ResponseId,
  httpContentType: String,
  eventContext: Option<EventContext>,
}
#[derive(Deserialize)]
struct ResponseBodySet {
  responseId: ResponseId,
  // bodyDescriptor: ShapedBodyDescriptor,
  eventContext: Option<EventContext>,
}
#[derive(Deserialize)]
struct ResponseBodyUnset {
  responseId: ResponseId,
  eventContext: Option<EventContext>,
}
#[derive(Deserialize)]
struct ResponseRemoved {
  responseId: ResponseId,
  eventContext: Option<EventContext>,
}
