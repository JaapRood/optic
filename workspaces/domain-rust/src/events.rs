struct EventContext {
  clientId: String,
  clientSessionId: String,
  clientCommandBatchId: String,
  createdAt: String,
}

pub enum Events {
  RfcEvent,
  RequestsEvent,
}

// RFC Events
// -----------

enum RfcEvent {
  ContributionAdded,
  APINamed,
  GitStateSet,
  BatchCommitStarted,
  BatchCommitEnded,
}

struct ContributionAdded {
  id: String,
  key: String,
  value: String,
  eventContext: Option<EventContext>,
}

struct APINamed {
  name: String,
  eventcontext: Option<EventContext>,
}

struct GitStateSet {
  branchName: String,
  commitId: String,
  eventContext: Option<EventContext>,
}

struct BatchCommitStarted {
  batchId: String,
  commitMessage: String,
  eventContext: Option<EventContext>,
}

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
struct PathComponentAdded {
  pathId: PathComponentId,
  parentPathId: PathComponentId,
  name: String,
  eventContext: Option<EventContext>,
}
struct PathComponentRenamed {
  pathId: PathComponentId,
  name: String,
  eventContext: Option<EventContext>,
}
struct PathComponentRemoved {
  pathId: PathComponentId,
  eventContext: Option<EventContext>,
}

// path parameters
struct PathParameterAdded {
  pathId: PathComponentId,
  parentPathId: PathComponentId,
  name: String,
  eventContext: Option<EventContext>,
}
// TODO: add PathParameterShapeSet
struct PathParameterRenamed {
  pathId: PathComponentId,
  name: String,
  eventContext: Option<EventContext>,
}
struct PathParameterRemoved {
  pathId: PathComponentId,
  name: String,
  eventContext: Option<EventContext>,
}

// request parameters
struct RequestParameterAddedByPathAndMethod {
  parameterId: RequestParameterId,
  pathId: PathComponentId,
  httpMethod: String,
  parameterLocation: String,
  name: String,
  eventContext: Option<EventContext>,
}
struct RequestParameterRenamed {
  parameterId: RequestParameterId,
  name: String,
  eventContext: Option<EventContext>,
}
// struct RequestParameterShapeSet {
//   parameterId: RequestParameterId,
//   parameterDescriptor: ShapedRequestParameterShapeDescriptor,
//   eventContext: Option<EventContext>,
// }
struct RequestParameterShapeUnset {
  parameterId: RequestParameterId,
  eventContext: Option<EventContext>,
}
struct RequestParameterRemoved {
  parameterId: RequestParameterId,
  eventContext: Option<EventContext>,
}

// Request events
struct RequestAdded {
  requestId: RequestId,
  pathId: PathComponentId,
  httpMethod: String,
  eventContext: Option<EventContext>,
}
struct RequestContentTypeSet {
  requestId: RequestId,
  httpContentType: String,
  eventContext: Option<EventContext>,
}
struct RequestBodySet {
  requestId: RequestId,
  // bodyDescriptor: ShapedBodyDescriptor,
  eventContext: Option<EventContext>,
}
struct RequestBodyUnset {
  requestId: RequestId,
  eventContext: Option<EventContext>,
}
struct RequestRemoved {
  requestId: RequestId,
  eventContext: Option<EventContext>,
}

// Response events
struct ResponseAddedByPathAndMethod {
  responseId: ResponseId,
  pathId: PathComponentId,
  httpMethod: String,
  httpStatusCode: u8,
  eventContext: Option<EventContext>,
}
struct ResponseStatusCodeSet {
  responseId: ResponseId,
  httpStatusCode: u8,
  eventContext: Option<EventContext>,
}
struct ResponseContentTypeSet {
  responseId: ResponseId,
  httpContentType: String,
  eventContext: Option<EventContext>,
}
struct ResponseBodySet {
  responseId: ResponseId,
  // bodyDescriptor: ShapedBodyDescriptor,
  eventContext: Option<EventContext>,
}
struct ResponseBodyUnset {
  responseId: ResponseId,
  eventContext: Option<EventContext>,
}
struct ResponseRemoved {
  responseId: ResponseId,
  eventContext: Option<EventContext>,
}
