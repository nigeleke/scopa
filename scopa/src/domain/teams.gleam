import gleam/dynamic/decode.{type Decoder}
import gleam/json.{type Json}
import gleam/list
import gleam/option.{type Option}

import domain/rounds.{type Rounds}
import domain/score.{type Score}
import domain/target.{type Target}
import domain/team.{type Team}
import domain/team/status

pub opaque type Teams {
  Teams(List(Team))
}

pub fn new() -> Teams {
  Teams(list.new())
}

pub fn value(teams: Teams) -> List(Team) {
  let Teams(teams) = teams
  teams
}

pub fn encode(teams: Teams) -> Json {
  let Teams(teams) = teams
  json.array(teams, team.encode)
}

pub fn decode() -> Decoder(Teams) {
  decode.list(team.decode())
  |> decode.map(Teams)
}

pub fn append(teams: Teams, team: Team) -> Teams {
  let Teams(teams) = teams
  Teams(teams |> list.append([team]))
}

pub fn update_scores(teams: Teams, rounds: Rounds) -> Teams {
  let Teams(teams) = teams

  let teams =
    teams
    |> list.map(fn(team) {
      let score = rounds.team_score(rounds, team.id)
      team.Team(..team, score:)
    })

  Teams(teams)
}

pub fn leading_score(teams: Teams) -> Score {
  let Teams(teams) = teams
  teams |> list.map(team.score) |> score.max
}

pub fn winner(teams: Teams, target: Target) -> Option(Team) {
  let Teams(teams) = teams

  let candidates =
    teams
    |> list.filter_map(fn(team) {
      case score.value(team.score) >= target.value(target) {
        True -> Ok(team)
        False -> Error(Nil)
      }
    })

  let candidates_count = list.length(candidates)

  case list.first(candidates) {
    Ok(team) if candidates_count == 1 -> option.Some(team)
    _ -> option.None
  }
}

pub fn eliminate_losers(teams: Teams, target: Target) -> Teams {
  let leading_score = leading_score(teams)

  let Teams(teams) = teams

  let eliminate_losers = fn(team) {
    case team.score(team) == leading_score {
      True -> team
      False -> team.Team(..team, status: status.Eliminated)
    }
  }

  let teams = case score.value(leading_score) >= target.value(target) {
    True -> teams |> list.map(eliminate_losers)
    False -> teams
  }

  Teams(teams)
}
