import gleam/dynamic/decode.{type Decoder}
import gleam/json.{type Json}
import gleam/list
import lustre/attribute as a
import lustre/element.{type Element}
import lustre/element/html as h

import domain/score
import domain/target.{type Target}
import domain/team.{type Team}
import domain/team/name
import domain/teams.{type Teams}
import ui/action/keep_teams
import ui/action/restart
import ui/glow

pub type Model {
  Model(teams: Teams, winner: Team, target: Target, keep_teams: Bool)
}

pub fn init(teams: Teams, winner: Team, target: Target) -> Model {
  Model(teams:, winner:, target:, keep_teams: True)
}

pub fn encode(model: Model) -> Json {
  json.object([
    #("teams", teams.encode(model.teams)),
    #("winner", team.encode(model.winner)),
    #("target", target.encode(model.target)),
    #("keep_teams", json.bool(model.keep_teams)),
  ])
}

pub fn decode() -> Decoder(Model) {
  use teams <- decode.field("teams", teams.decode())
  use winner <- decode.field("winner", team.decode())
  use target <- decode.field("target", target.decode())
  use keep_teams <- decode.field("keep_teams", decode.bool)
  decode.success(Model(teams, winner, target, keep_teams))
}

pub fn view(
  model: Model,
  on_update_keep_teams: fn(Bool) -> msg,
  on_restart: msg,
) -> Element(msg) {
  h.div([a.class("completed")], [
    winner(model.winner),
    team_results(model.teams),
    start_again(model.keep_teams, on_update_keep_teams, on_restart),
  ])
}

pub fn winner(team: Team) -> Element(msg) {
  let name = name.to_string(team.name)
  let winner = "Winner " <> name <> " !!!"
  h.span([a.class("completed__winner")], [glow.view([h.text(winner)])])
}

pub fn team_results(teams: Teams) -> Element(msg) {
  let results =
    teams
    |> teams.value
    |> list.flat_map(fn(team) {
      let name = name.to_string(team.name)
      let score = score.to_string(team.score)
      [
        h.span([a.class("completed__team-results__name")], [h.text(name)]),
        h.span([a.class("completed__team-results__score")], [h.text(score)]),
      ]
    })

  h.div([a.class("completed__team-results")], results)
}

pub fn start_again(
  keep_teams: Bool,
  on_update_keep_teams: fn(Bool) -> msg,
  on_restart: msg,
) -> Element(msg) {
  h.div([a.class("completed__start-again")], [
    restart.action(on_restart),
    keep_teams.action(keep_teams, on_update_keep_teams),
  ])
}
