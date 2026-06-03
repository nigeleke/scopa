import gleam/dynamic/decode.{type Decoder}
import gleam/json.{type Json}

pub type TeamStatus {
  Active
  Eliminated
}

pub fn encode(status: TeamStatus) -> Json {
  case status {
    Active -> json.string("active")
    Eliminated -> json.string("eliminated")
  }
}

pub fn decode() -> Decoder(TeamStatus) {
  use decoded_string <- decode.then(decode.string)

  case decoded_string {
    "active" -> decode.success(Active)
    "eliminated" -> decode.success(Eliminated)
    _ -> decode.failure(Active, "TeamStatus")
  }
}
