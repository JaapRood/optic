pub struct RequestsState {}

type PathComponentId = String;

struct PathComponent {
  pathId: PathComponentId,
  descriptor: PathComponentDescriptor,
  isRemoved: bool,
}

struct PathComponentDescriptor {
  parentPathId: PathComponentId,
  name: String,
}
