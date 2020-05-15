#[derive(Deserialize)]
struct EventContext {
  clientId: String,
  clientSessionId: String,
  clientCommandBatchId: String,
  createdAt: String,
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
  ContributionAdded {
    id: String,
    key: String,
    value: String,
    eventContext: Option<EventContext>,
  },
  APINamed {
    name: String,
    eventcontext: Option<EventContext>,
  },

  GitStateSet {
    branchName: String,
    commitId: String,
    eventContext: Option<EventContext>,
  },

  BatchCommitStarted {
    batchId: String,
    commitMessage: String,
    eventContext: Option<EventContext>,
  },

  BatchCommitEnded {
    batchId: String,
    eventContext: Option<EventContext>,
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
  PathComponentAdded {
    pathId: PathComponentId,
    parentPathId: PathComponentId,
    name: String,
    eventContext: Option<EventContext>,
  },
  PathComponentRenamed {
    pathId: PathComponentId,
    name: String,
    eventContext: Option<EventContext>,
  },
  PathComponentRemoved {
    pathId: PathComponentId,
    eventContext: Option<EventContext>,
  },

  // path parameters
  PathParameterAdded {
    pathId: PathComponentId,
    parentPathId: PathComponentId,
    name: String,
    eventContext: Option<EventContext>,
  },
  // TODO: add PathParameterShapeSet
  PathParameterRenamed {
    pathId: PathComponentId,
    name: String,
    eventContext: Option<EventContext>,
  },
  PathParameterRemoved {
    pathId: PathComponentId,
    name: String,
    eventContext: Option<EventContext>,
  },

  // request parameters
  RequestParameterAddedByPathAndMethod {
    parameterId: RequestParameterId,
    pathId: PathComponentId,
    httpMethod: String,
    parameterLocation: String,
    name: String,
    eventContext: Option<EventContext>,
  },
  RequestParameterRenamed {
    parameterId: RequestParameterId,
    name: String,
    eventContext: Option<EventContext>,
  },
  RequestParameterShapeSet {
    parameterId: RequestParameterId,
    // parameterDescriptor: ShapedRequestParameterShapeDescriptor,
    eventContext: Option<EventContext>,
  },
  RequestParameterShapeUnset {
    parameterId: RequestParameterId,
    eventContext: Option<EventContext>,
  },
  RequestParameterRemoved {
    parameterId: RequestParameterId,
    eventContext: Option<EventContext>,
  },

  // Request events
  RequestAdded {
    requestId: RequestId,
    pathId: PathComponentId,
    httpMethod: String,
    eventContext: Option<EventContext>,
  },
  RequestContentTypeSet {
    requestId: RequestId,
    httpContentType: String,
    eventContext: Option<EventContext>,
  },
  RequestBodySet {
    requestId: RequestId,
    // bodyDescriptor: ShapedBodyDescriptor,
    eventContext: Option<EventContext>,
  },
  RequestBodyUnset {
    requestId: RequestId,
    eventContext: Option<EventContext>,
  },
  RequestRemoved {
    requestId: RequestId,
    eventContext: Option<EventContext>,
  },

  // Response events
  ResponseAddedByPathAndMethod {
    responseId: ResponseId,
    pathId: PathComponentId,
    httpMethod: String,
    httpStatusCode: u8,
    eventContext: Option<EventContext>,
  },
  ResponseStatusCodeSet {
    responseId: ResponseId,
    httpStatusCode: u8,
    eventContext: Option<EventContext>,
  },

  ResponseContentTypeSet {
    responseId: ResponseId,
    httpContentType: String,
    eventContext: Option<EventContext>,
  },
  ResponseBodySet {
    responseId: ResponseId,
    // bodyDescriptor: ShapedBodyDescriptor,
    eventContext: Option<EventContext>,
  },
  ResponseBodyUnset {
    responseId: ResponseId,
    eventContext: Option<EventContext>,
  },
  ResponseRemoved {
    responseId: ResponseId,
    eventContext: Option<EventContext>,
  },
}

// Shape events
// ------------

type ShapeId = String;
type ShapeParameterId = String;
type FieldId = String;

#[derive(Deserialize)]
enum ShapeEvent {
  ShapeAdded {
    shapeId: ShapeId,
    baseShapeId: ShapeId,
    // parameters: ShapeParametersDescriptor,
    name: String,
    eventContext: Option<EventContext>,
  },
  BaseShapeSet {
    shapeId: ShapeId,
    baseShapeId: ShapeId,
    eventContext: Option<EventContext>,
  },
  ShapeRenamed {
    shapeId: ShapeId,
    name: String,
    eventContext: Option<EventContext>,
  },
  ShapeRemoved {
    shapeId: ShapeId,
    eventContext: Option<EventContext>,
  },

  ShapeParameterAdded {
    shapeParameterId: ShapeParameterId,
    shapeId: ShapeId,
    name: String,
    // shapeDescriptor: ParameterShapeDescriptor,
    eventContext: Option<EventContext>,
  },
  ShapeParameterShapeSet {
    // shapeDescriptor: ParameterShapeDescriptor,
    eventContext: Option<EventContext>,
  },
  ShapeParameterRenamed {
    shapeParameterId: ShapeParameterId,
    name: String,
    eventContext: Option<EventContext>,
  },
  ShapeParameterRemoved {
    shapeParameterId: ShapeParameterId,
    eventContext: Option<EventContext>,
  },

  FieldAdded {
    fieldId: FieldId,
    shapeId: ShapeId,
    name: String,
    // shapeDescriptor: FieldShapeDescriptor,
    eventContext: Option<EventContext>,
  },
  FieldShapeSet {
    // shapeDescriptor: FieldShapeDescriptor,
    eventContext: Option<EventContext>,
  },
  FieldRenamed {
    fieldId: FieldId,
    name: String,
    eventContext: Option<EventContext>,
  },
  FieldRemoved {
    fieldId: FieldId,
    eventContext: Option<EventContext>,
  },
}
