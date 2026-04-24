import domain/score
import gleam/list
import lustre
import lustre/element.{type Element}
import lustre/element/html.{button, div, text}
import lustre/event.{on_click}

import domain/round.{type Round}
import domain/target.{type Target}
import domain/team.{type Team}
import domain/team_name.{type TeamName}
import domain/team_status
import ui/footer
import ui/header
import ui/in_progress
import ui/setup

pub type Model {
  Setup(candidate_name: TeamName, team_names: List(TeamName), target: Target)
  InProgress(teams: List(Team), rounds: List(Round), target: Target)
  Completed(teams: List(Team), target: Target, winner: Team)
}

pub type Message {
  CandidateNameChanged(TeamName)
  TeamAdded(TeamName)
  TeamsRemoved(index: Int)
  TargetChanged(Target)
  GameStarted
  RoundScored(Round)
  WinnerAnnounced(Team)
  RestartWithSameSetup
  ReturnToSetup
}

fn init(_flags: Nil) -> Model {
  Setup(team_name.new(""), list.new(), target.from_int(11))
}

fn update(model: Model, message: Message) -> Model {
  case message {
    CandidateNameChanged(candidate_name) ->
      model |> update_candidate_name(candidate_name)
    TeamAdded(team) -> model |> add_team(team)
    TeamsRemoved(index) -> model |> remove_team(index)
    TargetChanged(value) -> model |> change_target(value)
    GameStarted -> model |> start_game()
    RoundScored(_) -> todo
    WinnerAnnounced(_) -> todo
    RestartWithSameSetup -> todo
    ReturnToSetup -> todo
  }
}

fn update_candidate_name(model: Model, candidate_name) -> Model {
  let assert Setup(_, _, _) = model
  Setup(..model, candidate_name:)
}

fn add_team(model: Model, team: TeamName) -> Model {
  let assert Setup(_, team_names, _) = model
  let candidate_name = team_name.new("")
  let team_names = team_names |> list.append([team])
  Setup(..model, candidate_name:, team_names:)
}

fn remove_team(model: Model, index: Int) -> Model {
  let assert Setup(_, team_names, _) = model
  let team_names =
    team_names
    |> list.index_fold(list.new(), fn(acc, item, i) {
      case i == index {
        True -> acc
        False -> acc |> list.append([item])
      }
    })
  Setup(..model, team_names:)
}

fn change_target(model: Model, target: Target) -> Model {
  let assert Setup(_, _, _) = model
  Setup(..model, target:)
}

fn start_game(model: Model) -> Model {
  let assert Setup(_, team_names, target) = model
  let teams =
    team_names |> list.map(team.Team(_, score.from_int(0), team_status.Active))
  InProgress(list.new(), teams:, target:)
}

pub fn view(model: Model) -> Element(Message) {
  element.fragment([
    header.view(),
    main_content(model),
    footer.view(),
  ])
}

fn main_content(model: Model) -> Element(Message) {
  let view = case model {
    Setup(candidate_name, team_names, target) ->
      setup.view(
        setup.Model(candidate_name, team_names, target),
        setup.Props(
          CandidateNameChanged,
          TeamAdded,
          TeamsRemoved,
          TargetChanged,
          GameStarted,
        ),
      )

    InProgress(teams, rounds, target) ->
      in_progress.view(
        in_progress.Model(teams, rounds, target),
        in_progress.Props(RoundScored),
      )

    Completed(_, _, _) ->
      div([], [
        text("Game Over! Winner: "),
        button([on_click(ReturnToSetup)], [
          text("Play Again"),
        ]),
      ])
  }

  html.main([], [view])
}

pub fn main() {
  let app = lustre.simple(init, update, view)
  let assert Ok(_) = lustre.start(app, "#app", Nil)
  Nil
}
