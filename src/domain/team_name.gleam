pub opaque type TeamName {
  TeamName(String)
}

pub fn new(value: String) -> TeamName {
  TeamName(value)
}

pub fn to_string(self: TeamName) -> String {
  let TeamName(value) = self
  value
}
