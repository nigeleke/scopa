import gleam/list
import gleam/option.{type Option}

import domain/rounds.{type Rounds}
import domain/score.{type Score}
import domain/target.{type Target}
import domain/team.{type Team}
import domain/team/id.{type TeamId}
import domain/team/name.{type TeamName}
import domain/team/status.{type TeamStatus}
import domain/teams.{type Teams}

pub type TeamScore =
  #(Team, Score)

pub fn team_scores(teams: Teams, rounds: Rounds) -> List(TeamScore) {
  teams
  |> teams.map(fn(team) { new(team, rounds |> rounds.team_score(team.id)) })
}

pub fn new(team: Team, score: Score) -> TeamScore {
  #(team, score)
}

pub fn team(team_score: TeamScore) -> Team {
  let #(team, _) = team_score
  team
}

pub fn team_id(team_score: TeamScore) -> TeamId {
  let #(team, _) = team_score
  team.id
}

pub fn team_name(team_score: TeamScore) -> TeamName {
  let #(team, _) = team_score
  team.name
}

pub fn team_status(team_score: TeamScore) -> TeamStatus {
  let #(team, _) = team_score
  team.status
}

pub fn score(team_score: TeamScore) -> Score {
  let #(_, score) = team_score
  score
}

pub fn leading_score(team_scores: List(TeamScore)) -> Score {
  team_scores |> list.map(score) |> score.max
}

pub fn winner(team_scores: List(TeamScore), target: Target) -> Option(Team) {
  let leading_score = leading_score(team_scores)
  let winning_teams =
    team_scores
    |> list.filter_map(fn(team_score) {
      let #(team, score) = team_score

      let is_leading = score == leading_score
      let is_winning = score.value(score) >= target.value(target)

      case is_leading && is_winning {
        True -> Ok(team)
        _ -> Error(Nil)
      }
    })

  let winning_teams_count = list.length(winning_teams)
  case list.first(winning_teams) {
    Ok(team) if winning_teams_count == 1 -> option.Some(team)
    _ -> option.None
  }
}
