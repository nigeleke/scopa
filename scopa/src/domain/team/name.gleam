import gleam/dynamic/decode.{type Decoder}
import gleam/json.{type Json}
import gleam/list

pub opaque type TeamName {
  TeamName(String)
}

pub fn new(value: String) -> TeamName {
  TeamName(value)
}

pub fn to_string(name: TeamName) -> String {
  let TeamName(value) = name
  value
}

pub fn has_valid_team_count(names: List(TeamName)) -> Bool {
  let teams_count = list.length(names)
  [2, 3, 4, 6] |> list.contains(teams_count)
}

pub fn encode(name: TeamName) -> Json {
  json.string(to_string(name))
}

pub fn decode() -> Decoder(TeamName) {
  decode.string |> decode.map(new)
}
