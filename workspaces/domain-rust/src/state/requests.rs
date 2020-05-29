use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct RequestsState {
  pathComponents: HashMap<PathComponentId, PathComponent>,
}

pub type PathComponentId = String;

#[derive(Debug)]
struct PathComponent {
  pathId: PathComponentId,
  descriptor: PathComponentDescriptor,
  isRemoved: bool,
}

#[derive(Debug)]
struct PathComponentDescriptor {
  parentPathId: PathComponentId,
  name: String,
}
