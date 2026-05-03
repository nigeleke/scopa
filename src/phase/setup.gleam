import domain/target
import gleam/list
import lustre/attribute
import lustre/element.{type Element}
import lustre/element/html
import lustre/event

import domain/team/name.{type TeamName}
import ui/target as target_ui
import ui/team_names_editor

pub type Model {
  Model(raw_team_name: String, team_names: List(TeamName), raw_target: String)
}

pub fn init() -> Model {
  Model(raw_team_name: "", team_names: list.new(), raw_target: "11")
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

  html.div([attribute.class("setup")], [
    target_ui.editor(model.raw_target, on_raw_target_change),
    team_names_editor.view(
      model.raw_team_name,
      model.team_names,
      on_raw_team_name_change,
      on_team_added,
      on_team_removed,
    ),
    html.button([attribute.disabled(!can_start), event.on_click(on_start)], [
      html.text("Start"),
    ]),
  ])
}
