import lustre/attribute
import lustre/element.{type Element}
import lustre/element/html

import derived/team_score
import domain/rounds.{type Rounds}
import domain/tally.{type Tally}
import domain/target.{type Target}
import domain/teams.{type Teams}
import ui/action/score as score_action
import ui/tally_editor

pub type Model {
  Model(teams: Teams, draft_tally: Tally, rounds: Rounds, target: Target)
}

pub fn init(teams: Teams, target: Target) -> Model {
  Model(teams:, draft_tally: tally.new(), rounds: rounds.new(), target:)
}

pub fn view(
  model: Model,
  on_draft_tally_changed: fn(Tally) -> msg,
  on_score: msg,
) -> Element(msg) {
  let round = rounds.round_number(model.rounds)
  let team_scores = team_score.team_scores(model.teams, model.rounds)
  let draft_tally = model.draft_tally

  html.div([attribute.class("in-progress")], [
    tally_editor.view(round, team_scores, draft_tally, on_draft_tally_changed),
    score_action.action(model.draft_tally, on_score),
  ])
}
