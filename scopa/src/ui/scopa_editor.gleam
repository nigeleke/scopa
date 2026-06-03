import gleam/dynamic/decode.{type Decoder}
import gleam/int
import gleam/json.{type Json}
import lustre/attribute as a
import lustre/element.{type Element}
import lustre/element/html as h

import domain/score.{type Score}
import domain/team/id.{type TeamId} as team_id
import ui/action/cancel_scopa_score_edit
import ui/action/edit_scopa_score_more
import ui/action/submit_scopa_score

pub type Model {
  Closed
  OpenPartial(TeamId)
  OpenExpanded(TeamId)
}

pub fn encode(model: Model) -> Json {
  case model {
    Closed ->
      json.object([
        #("type", json.string("closed")),
      ])

    OpenPartial(team_id) ->
      json.object([
        #("type", json.string("open_partial")),
        #("team_id", team_id.encode(team_id)),
      ])

    OpenExpanded(team_id) ->
      json.object([
        #("type", json.string("open_expanded")),
        #("team_id", team_id.encode(team_id)),
      ])
  }
}

pub fn decode() -> Decoder(Model) {
  use kind <- decode.field("type", decode.string)

  case kind {
    "closed" -> decode.success(Closed)
    "open_partial" ->
      decode.field("team_id", team_id.decode(), fn(id) {
        decode.success(OpenPartial(id))
      })
    "open_expanded" ->
      decode.field("team_id", team_id.decode(), fn(id) {
        decode.success(OpenExpanded(id))
      })
    _ -> decode.failure(Closed, "Model")
  }
}

pub fn view(
  model: Model,
  on_more: fn(TeamId) -> msg,
  on_submit: fn(TeamId, Score) -> msg,
  on_cancel: msg,
) -> Element(msg) {
  case model {
    Closed -> element.none()

    OpenPartial(team_id) ->
      editor(team_id, model, on_more, on_submit, on_cancel)

    OpenExpanded(team_id) ->
      editor(team_id, model, on_more, on_submit, on_cancel)
  }
}

fn editor(
  team_id: TeamId,
  model: Model,
  on_more: fn(TeamId) -> msg,
  on_submit: fn(TeamId, Score) -> msg,
  on_cancel: msg,
) -> Element(msg) {
  let on_more = on_more(team_id)
  let on_submit = on_submit(team_id, _)

  h.div([a.class("scopa-editor")], [
    buttons_dialog(model, on_more, on_submit, on_cancel),
  ])
}

fn buttons_dialog(
  model: Model,
  on_more: msg,
  on_submit: fn(Score) -> msg,
  on_cancel: msg,
) -> Element(msg) {
  h.dialog([a.class("scopa-editor__buttons-dialog"), a.open(True)], [
    buttons_panel(model, on_submit),
    actions_panel(model, on_more, on_cancel),
  ])
}

fn buttons_panel(model: Model, on_submit: fn(Score) -> msg) -> Element(msg) {
  let score_buttons_generator = case model {
    Closed -> int.range(0, 0, [], _)
    OpenPartial(_) -> int.range(5, -1, [], _)
    OpenExpanded(_) -> int.range(13, -1, [], _)
  }

  let score_buttons =
    score_buttons_generator(fn(acc, n) {
      let score = score.from_int(n)
      let action = submit_scopa_score.action(score, on_submit)
      [action, ..acc]
    })

  h.div([a.class("scopa-editor__buttons-panel")], score_buttons)
}

fn actions_panel(model: Model, on_more: msg, on_cancel: msg) -> Element(msg) {
  h.div([a.class("scopa-editor__actions-panel")], [
    case model {
      OpenPartial(_) -> edit_scopa_score_more.action(on_more)
      _ -> element.none()
    },
    cancel_scopa_score_edit.action(on_cancel),
  ])
}
