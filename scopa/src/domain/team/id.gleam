import gleam/dynamic/decode.{type Decoder}
import gleam/json.{type Json}

pub opaque type TeamId {
  TeamId(Int)
}

pub fn new(value: Int) -> TeamId {
  TeamId(value)
}

pub fn encode(id: TeamId) -> Json {
  let TeamId(id) = id
  json.int(id)
}

pub fn decode() -> Decoder(TeamId) {
  decode.int |> decode.map(new)
}
