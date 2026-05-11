import gleam/dynamic/decode.{type Decoder}
import gleam/json.{type Json}
import gleam/list
import lustre/attribute as a
import lustre/element.{type Element}
import lustre/element/html as h

import domain/target
import domain/team/name.{type TeamName}
import ui/action/start
import ui/target as target_ui
import ui/team_names_editor

pub type Model {
  Model(raw_team_name: String, team_names: List(TeamName), raw_target: String)
}

pub fn init() -> Model {
  Model(raw_team_name: "", team_names: list.new(), raw_target: "11")
}

pub fn encode(model: Model) -> Json {
  json.object([
    #("raw_team_name", json.string(model.raw_team_name)),
    #("team_names", json.array(model.team_names, name.encode)),
    #("raw_target", json.string(model.raw_target)),
  ])
}

pub fn decode() -> Decoder(Model) {
  use raw_team_name <- decode.field("raw_team_name", decode.string)
  use team_names <- decode.field("team_names", decode.list(name.decode()))
  use raw_target <- decode.field("raw_target", decode.string)
  decode.success(Model(raw_team_name:, team_names:, raw_target:))
}

pub fn view(
  model: Model,
  on_raw_team_name_change: fn(String) -> msg,
  on_team_added: fn(TeamName) -> msg,
  on_team_removed: fn(Int) -> msg,
  on_raw_target_change: fn(String) -> msg,
  on_start: msg,
) -> Element(msg) {
  let can_start =
    name.has_valid_team_count(model.team_names)
    && target.is_valid_target(model.raw_target)

  echo "setup::view"
  echo model

  h.div([a.class("setup")], [
    target_ui.editor(model.raw_target, on_raw_target_change),
    team_names_editor.view(
      model.raw_team_name,
      model.team_names,
      on_raw_team_name_change,
      on_team_added,
      on_team_removed,
    ),
    start.action(!can_start, on_start),
  ])
}
