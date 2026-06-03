import gleam/dynamic/decode.{type Decoder}
import gleam/json.{type Json}

import domain/score.{type Score}
import domain/team/id.{type TeamId}
import domain/team/name.{type TeamName}
import domain/team/status.{type TeamStatus}

pub type Team {
  Team(id: TeamId, name: TeamName, score: Score, status: TeamStatus)
}

pub fn new(id: TeamId, name: TeamName) -> Team {
  Team(id, name, score: score.zero, status: status.Active)
}

pub fn encode(team: Team) -> Json {
  json.object([
    #("id", id.encode(team.id)),
    #("name", name.encode(team.name)),
    #("score", score.encode(team.score)),
    #("status", status.encode(team.status)),
  ])
}

pub fn decode() -> Decoder(Team) {
  use id <- decode.field("id", id.decode())
  use name <- decode.field("name", name.decode())
  use score <- decode.field("score", score.decode())
  use status <- decode.field("status", status.decode())

  decode.success(Team(id:, name:, score:, status:))
}

pub fn name(team: Team) -> TeamName {
  let Team(_, name, _, _) = team
  name
}

pub fn score(team: Team) -> Score {
  let Team(_, _, score, _) = team
  score
}
