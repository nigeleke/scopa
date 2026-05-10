import lustre/attribute as a
import lustre/element.{type Element}
import lustre/element/html as h

import domain/rounds.{type Rounds}
import domain/score.{type Score}
import domain/tally.{type Tally}
import domain/target.{type Target}
import domain/team/id.{type TeamId}
import domain/teams.{type Teams}
import ui/action/score as score_action
import ui/scopa_editor.{type Model as ScopaEditorModel}
import ui/tally_editor

pub type Model {
  Model(
    teams: Teams,
    draft_tally: Tally,
    scopa_editor: ScopaEditorModel,
    rounds: Rounds,
    target: Target,
  )
}

pub fn init(teams: Teams, target: Target) -> Model {
  Model(
    teams:,
    draft_tally: tally.new(),
    scopa_editor: scopa_editor.Closed,
    rounds: rounds.new(),
    target:,
  )
}

pub fn view(
  model: Model,
  on_draft_tally_changed: fn(Tally) -> msg,
  on_edit_scopa_score: fn(TeamId) -> msg,
  on_submit_scopa_score: fn(TeamId, Score) -> msg,
  on_cancel_scopa_score: msg,
  on_score: msg,
) -> Element(msg) {
  let round = rounds.round_number(model.rounds)
  let draft_tally = model.draft_tally

  h.div([a.class("in-progress")], [
    tally_editor.view(
      model.teams,
      round,
      draft_tally,
      on_draft_tally_changed,
      on_edit_scopa_score,
    ),
    score_action.action(model.draft_tally, on_score),
    scopa_editor.view(
      model.scopa_editor,
      on_edit_scopa_score,
      on_submit_scopa_score,
      on_cancel_scopa_score,
    ),
  ])
}
