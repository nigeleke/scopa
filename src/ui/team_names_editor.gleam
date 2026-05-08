import gleam/list
import lustre/attribute as a
import lustre/element.{type Element}
import lustre/element/html as h

import domain/team/name.{type TeamName}
import ui/action/add_team
import ui/action/remove_team
import ui/team_name as team_name_ui

pub fn view(
  raw_team_name: String,
  team_names: List(TeamName),
  on_raw_team_name_changed: fn(String) -> msg,
  on_team_added: fn(TeamName) -> msg,
  on_team_removed: fn(Int) -> msg,
) -> Element(msg) {
  h.div([a.class("team-names-editor")], [
    name_input(raw_team_name, on_raw_team_name_changed, on_team_added),
    teams_list(team_names, on_team_removed),
    ..error_view(team_names)
  ])
}

fn name_input(
  raw_team_name: String,
  on_raw_team_name_changed: fn(String) -> msg,
  on_team_added: fn(TeamName) -> msg,
) -> Element(msg) {
  h.div([a.class("team-names-editor__name-input")], [
    team_name_ui.editor(raw_team_name, on_raw_team_name_changed),
    add_team.action(on_team_added(name.new(raw_team_name))),
  ])
}

fn teams_list(
  team_names: List(TeamName),
  on_team_removed: fn(Int) -> msg,
) -> Element(msg) {
  let team_rows =
    team_names
    |> list.index_map(fn(name, index) { team_row(name, index, on_team_removed) })

  h.div([a.class("team-names-editor__teams-list-wrapper")], [
    h.div([a.class("team-names-editor__teams-list")], team_rows),
  ])
}

fn team_row(
  team_name: TeamName,
  index: Int,
  on_team_removed: fn(Int) -> msg,
) -> Element(msg) {
  element.fragment([
    remove_team.action(on_team_removed(index)),
    team_name_ui.view(team_name),
  ])
}

fn error_view(team_names: List(TeamName)) -> List(Element(msg)) {
  case name.has_valid_team_count(team_names) {
    True -> []
    False -> [
      h.span([a.class("team-names-editor__error")], [
        h.text("Enter 2, 3, 4 or 6 team names"),
      ]),
    ]
  }
}
