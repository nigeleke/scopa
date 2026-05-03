import gleam/list
import gleam/option
import lustre
import lustre/element.{type Element}
import lustre/element/html
import lustre/event

import derived/team_score
import domain/rounds
import domain/tally.{type Tally}
import domain/target
import domain/team
import domain/team/id
import domain/team/name.{type TeamName}
import domain/teams
import phase/completed.{type Model as CompletedModel}
import phase/in_progress.{type Model as InProgressModel}
import phase/setup.{type Model as SetupModel}
import ui/footer
import ui/header

pub type Model {
  Setup(SetupModel)
  InProgress(InProgressModel)
  Completed(CompletedModel)
}

pub type Message {
  // Setup messages
  ChangeRawTeamName(String)
  AddTeam(TeamName)
  RemoveTeam(index: Int)
  ChangeRawTarget(String)
  StartGame

  // InProgress messages
  ChangeDraftTally(tally: Tally)
  ScoreRound

  // Completed messages
  RestartGameWithSameSetup
  ResetGame
}

fn init(_flags: Nil) -> Model {
  Setup(setup.init())
}

fn update(model: Model, message: Message) -> Model {
  case message {
    // Setup messages
    ChangeRawTeamName(name) -> model |> change_raw_team_name(name)
    AddTeam(name) -> model |> add_team(name)
    RemoveTeam(index) -> model |> remove_team(index)
    ChangeRawTarget(target) -> model |> change_raw_target(target)
    StartGame -> model |> start_game()

    // InProgress messages
    ChangeDraftTally(tally) -> model |> change_draft_tally(tally)
    ScoreRound -> model |> score_round()

    // Completed messages
    RestartGameWithSameSetup -> model |> restart_game()
    ResetGame -> model |> reset_game()
  }
}

fn change_raw_team_name(model: Model, raw_team_name: String) -> Model {
  let assert Setup(model) = model
  Setup(setup.Model(..model, raw_team_name:))
}

fn add_team(model: Model, team: TeamName) -> Model {
  let assert Setup(model) = model

  let raw_team_name = ""
  let team_names = model.team_names |> list.append([team])

  Setup(setup.Model(..model, raw_team_name:, team_names:))
}

fn remove_team(model: Model, index: Int) -> Model {
  let assert Setup(model) = model

  let team_names =
    model.team_names
    |> list.index_fold(list.new(), fn(acc, item, i) {
      case i == index {
        True -> acc
        False -> acc |> list.append([item])
      }
    })

  Setup(setup.Model(..model, team_names:))
}

fn change_raw_target(model: Model, raw_target: String) -> Model {
  let assert Setup(model) = model
  Setup(setup.Model(..model, raw_target:))
}

fn start_game(model: Model) -> Model {
  let assert Setup(model) = model

  let teams =
    model.team_names
    |> list.index_fold(teams.new(), fn(acc, team_name, index) {
      let team = team.new(id.new(index), team_name)
      acc |> teams.append(team)
    })

  let assert Ok(target) = target.from_string(model.raw_target)

  InProgress(in_progress.init(teams, target))
}

fn change_draft_tally(model: Model, draft_tally: Tally) -> Model {
  let assert InProgress(model) = model
  InProgress(in_progress.Model(..model, draft_tally:))
}

fn score_round(model: Model) -> Model {
  let assert InProgress(model) = model

  let rounds = model.rounds |> rounds.append(model.draft_tally)
  let team_scores = team_score.team_scores(model.teams, rounds)

  case team_scores |> team_score.winner(model.target) {
    option.Some(winner) -> {
      let completed = completed.init(team_scores, winner, model.target)
      Completed(completed)
    }
    option.None -> {
      let draft_tally = tally.new()
      InProgress(in_progress.Model(..model, draft_tally:, rounds:))
    }
  }
}

fn restart_game(model: Model) -> Model {
  let assert Completed(model) = model

  let teams =
    model.team_scores
    |> list.fold(teams.new(), fn(acc, team_score) {
      let team = team_score.team(team_score)
      let team = team.new(team.id, team.name)
      acc |> teams.append(team)
    })

  InProgress(in_progress.init(teams, model.target))
}

fn reset_game(model: Model) -> Model {
  let assert Completed(_) = model
  Setup(setup.init())
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
    Setup(model) ->
      setup.view(
        model,
        ChangeRawTeamName,
        AddTeam,
        RemoveTeam,
        ChangeRawTarget,
        StartGame,
      )

    InProgress(model) -> in_progress.view(model, ChangeDraftTally, ScoreRound)

    Completed(_) ->
      html.div([], [
        html.text("Game Over! Winner: "),
        html.button([event.on_click(ResetGame)], [
          html.text("Play Again"),
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
