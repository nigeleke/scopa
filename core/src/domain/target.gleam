import gleam/int
import gleam/result

pub opaque type Target {
  Target(Int)
}

pub fn value(target: Target) -> Int {
  let Target(value) = target
  value
}

pub fn from_int(value: Int) -> Target {
  Target(value)
}

pub fn from_string(value: String) -> Result(Target, Nil) {
  case int.parse(value) {
    Ok(value) if value > 0 -> Ok(Target(value))
    _ -> Error(Nil)
  }
}

pub fn to_string(target: Target) -> String {
  let Target(value) = target
  int.to_string(value)
}

pub fn is_valid_target(value: String) -> Bool {
  result.is_ok(from_string(value))
}
