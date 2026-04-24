pub opaque type Score {
  Score(Int)
}

pub fn from_int(value: Int) -> Score {
  Score(value)
}
