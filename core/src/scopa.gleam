import gleam/list
import gleam/option
import lustre
import lustre/element.{type Element}
import lustre/element/html as h

import domain/rounds
import domain/score.{type Score}
import domain/tally.{type Tally}
import domain/target.{type Target}
import domain/team
import domain/team/id.{type TeamId}
import domain/team/name.{type TeamName}
import domain/teams.{type Teams}
import phase/completed.{type Model as CompletedModel}
import phase/in_progress.{type Model as InProgressModel}
import phase/setup.{type Model as SetupModel}
import ui/footer
import ui/header
import ui/scopa_editor

pub type Model {
  Setup(SetupModel)
  InProgress(InProgressModel)
  Completed(CompletedModel)
}

pub type Message {
  // Setup messages
  UpdateTeamNameInput(String)
  AddTeam(TeamName)
  RemoveTeam(index: Int)
  UpdateTargetInput(String)
  StartGame

  // InProgress messages
  SetDraftTally(tally: Tally)
  EditScopaScore(TeamId)
  EditScopaScoreMore(TeamId)
  SubmitScopaScore(TeamId, Score)
  CancelScopaScoreEdit
  ScoreRound

  // Completed messages
  UpdateKeepTeams(Bool)
  Restart
}

fn init(_flags: Nil) -> Model {
  Setup(setup.init())
}

fn update(model: Model, message: Message) -> Model {
  case echo message {
    // Setup messages
    UpdateTeamNameInput(name) -> model |> update_team_name_input(name)
    AddTeam(name) -> model |> add_team(name)
    RemoveTeam(index) -> model |> remove_team(index)
    UpdateTargetInput(target) -> model |> update_target_input(target)
    StartGame -> model |> start_game()

    // InProgress messages
    SetDraftTally(tally) -> model |> set_draft_tally(tally)
    EditScopaScore(team_id) -> model |> edit_scopa_score(team_id, False)
    EditScopaScoreMore(team_id) -> model |> edit_scopa_score(team_id, True)
    SubmitScopaScore(team_id, value) ->
      model |> submit_scopa_score(team_id, value)
    CancelScopaScoreEdit -> model |> cancel_scopa_score_edit()
    ScoreRound -> model |> score_round()

    // Completed messages
    UpdateKeepTeams(keep) -> model |> update_keep_teams(keep)
    Restart -> model |> restart
  }
}

fn update_team_name_input(model: Model, raw_team_name: String) -> Model {
  let assert Setup(model) = model
  echo Setup(setup.Model(..model, raw_team_name:))
}

fn add_team(model: Model, team_name: TeamName) -> Model {
  let assert Setup(model) = model

  let raw_team_name = ""
  let team_names = model.team_names |> list.append([team_name])

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

fn update_target_input(model: Model, raw_target: String) -> Model {
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

fn set_draft_tally(model: Model, draft_tally: Tally) -> Model {
  let assert InProgress(model) = model
  InProgress(in_progress.Model(..model, draft_tally:))
}

fn edit_scopa_score(model: Model, team_id: TeamId, full_reveal: Bool) -> Model {
  let assert InProgress(model) = model

  let scopa_editor = case full_reveal {
    False -> scopa_editor.OpenPartial(team_id)
    True -> scopa_editor.OpenExpanded(team_id)
  }
  InProgress(in_progress.Model(..model, scopa_editor:))
}

fn submit_scopa_score(model: Model, team_id: TeamId, score: Score) -> Model {
  let assert InProgress(model) = model
  let draft_tally =
    model.draft_tally
    |> tally.set_scopas(for: team_id, value: score)
  let scopa_editor = scopa_editor.Closed
  InProgress(in_progress.Model(..model, draft_tally:, scopa_editor:))
}

fn cancel_scopa_score_edit(model: Model) -> Model {
  let assert InProgress(model) = model
  let scopa_editor = scopa_editor.Closed
  InProgress(in_progress.Model(..model, scopa_editor:))
}

fn score_round(model: Model) -> Model {
  let assert InProgress(model) = model

  let rounds = model.rounds |> rounds.append(model.draft_tally)
  let teams = model.teams |> teams.update_scores(rounds)
  let winner = teams |> teams.winner(model.target)

  case winner {
    option.Some(winner) -> {
      let completed = completed.init(teams, winner, model.target)
      Completed(completed)
    }
    option.None -> {
      let teams = teams |> teams.eliminate_losers(model.target)
      let draft_tally = tally.new()
      InProgress(in_progress.Model(..model, teams:, draft_tally:, rounds:))
    }
  }
}

fn update_keep_teams(model: Model, keep_teams: Bool) -> Model {
  let assert Completed(model) = model
  Completed(completed.Model(..model, keep_teams:))
}

fn restart(model: Model) -> Model {
  let assert Completed(model) = model
  case model.keep_teams {
    True -> restart_with_teams(model.teams, model.target)
    False -> restart_new()
  }
}

fn restart_with_teams(teams: Teams, target: Target) -> Model {
  let teams =
    teams
    |> teams.value
    |> list.fold(teams.new(), fn(acc, team) {
      let team = team.new(team.id, team.name)
      acc |> teams.append(team)
    })

  InProgress(in_progress.init(teams, target))
}

fn restart_new() -> Model {
  Setup(setup.init())
}

pub fn view(model: Model) -> Element(Message) {
  echo "scopa::view"
  echo model
  element.fragment([
    header.view(),
    main_content(model),
    footer.view(),
  ])
}

fn main_content(model: Model) -> Element(Message) {
  echo "main_content"
  echo model

  let view = case model {
    Setup(model) ->
      setup.view(
        model,
        UpdateTeamNameInput,
        AddTeam,
        RemoveTeam,
        UpdateTargetInput,
        StartGame,
      )

    InProgress(model) ->
      in_progress.view(
        model,
        SetDraftTally,
        case model.scopa_editor {
          scopa_editor.Closed -> EditScopaScore
          _ -> EditScopaScoreMore
        },
        SubmitScopaScore,
        CancelScopaScoreEdit,
        ScoreRound,
      )

    Completed(model) -> completed.view(model, UpdateKeepTeams, Restart)
  }

  h.main([], [view])
}

pub fn main() {
  let app = lustre.simple(init, update, view)
  let assert Ok(_) = lustre.start(app, "#app", Nil)
  Nil
}
