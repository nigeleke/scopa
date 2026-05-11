import gleam/dynamic/decode.{type Decoder}
import gleam/json.{type Json}
import gleam/list

import domain/score.{type Score}
import domain/tally.{type Tally}
import domain/team/id.{type TeamId}

pub opaque type Rounds {
  Rounds(List(Tally))
}

pub fn new() -> Rounds {
  Rounds(list.new())
}

pub fn encode(rounds: Rounds) -> Json {
  let Rounds(tallies) = rounds
  json.array(tallies, tally.encode)
}

pub fn decode() -> Decoder(Rounds) {
  decode.list(tally.decode())
  |> decode.map(Rounds)
}

pub fn round_number(rounds: Rounds) -> Int {
  let Rounds(rounds) = rounds
  list.length(rounds) + 1
}

pub fn append(rounds: Rounds, tally: Tally) -> Rounds {
  let Rounds(rounds) = rounds
  Rounds(rounds |> list.append([tally]))
}

pub fn team_score(rounds: Rounds, team_id: TeamId) -> Score {
  let Rounds(rounds) = rounds
  rounds
  |> list.map(fn(round) { tally.score(round, team_id) })
  |> score.sum
}
