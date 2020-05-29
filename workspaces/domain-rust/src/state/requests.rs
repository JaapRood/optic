use std::collections::HashMap;

#[derive(Default)]
pub struct RequestsState {
  pathComponents: HashMap<PathComponentId, PathComponent>,
}

pub type PathComponentId = String;

struct PathComponent {
  pathId: PathComponentId,
  descriptor: PathComponentDescriptor,
  isRemoved: bool,
}

struct PathComponentDescriptor {
  parentPathId: PathComponentId,
  name: String,
}
