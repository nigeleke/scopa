import gleam/dynamic/decode.{type Decoder}
import gleam/int
import gleam/json.{type Json}
import gleam/list

pub opaque type Score {
  Score(Int)
}

pub fn value(score: Score) -> Int {
  let Score(value) = score
  value
}

pub const zero: Score = Score(0)

pub fn from_int(value: Int) -> Score {
  case value >= 0 {
    True -> Score(value)
    False -> panic as { "score must be >= zero, was " <> int.to_string(value) }
  }
}

pub fn encode(score: Score) -> Json {
  let Score(value) = score
  json.int(value)
}

pub fn decode() -> Decoder(Score) {
  use value <- decode.then(decode.int)
  decode.success(Score(value))
}

fn add(a: Score, b: Score) -> Score {
  Score(value(a) + value(b))
}

pub fn sum(scores: List(Score)) -> Score {
  list.fold(scores, zero, add)
}

pub fn max(scores: List(Score)) -> Score {
  let max_value =
    scores
    |> list.map(value)
    |> list.max(int.compare)

  case max_value {
    Ok(value) -> Score(value)
    Error(_) -> zero
  }
}

pub fn to_string(score: Score) -> String {
  let Score(value) = score
  int.to_string(value)
}
