import gleam/int

pub opaque type Target {
  Target(Int)
}

pub fn from_int(value: Int) -> Target {
  Target(value)
}

pub fn to_string(self: Target) -> String {
  let Target(value) = self
  int.to_string(value)
}
