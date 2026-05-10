pub opaque type TeamId {
  TeamId(Int)
}

pub fn new(value: Int) -> TeamId {
  TeamId(value)
}
