pub mod requests;

use requests::RequestsState;

struct RfcState {
  requestsState: RequestsState,
  shapesState: ShapesState,
  scmState: ScmState,
}

struct ShapesState {}
struct ScmState {}
