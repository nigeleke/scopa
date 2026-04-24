import gleam/list
import lustre/attribute.{autofocus, class, id, placeholder, tabindex, value}
import lustre/element.{type Element}
import lustre/element/html.{button, div, input, span, text}
import lustre/event.{on_change, on_click}

import domain/team_name.{type TeamName}

pub type Model {
  Model(candidate_name: TeamName, team_names: List(TeamName))
}

pub type Props(msg) {
  Props(
    on_candidate_name_changed: fn(TeamName) -> msg,
    on_team_added: fn(TeamName) -> msg,
    on_team_removed: fn(Int) -> msg,
  )
}

pub fn view(model: Model, props: Props(msg)) -> Element(msg) {
  div([class("team-names-editor")], [
    name_input(model, props),
    teams_list(model, props),
  ])
}

fn name_input(model: Model, props: Props(msg)) -> Element(msg) {
  let on_candidate_name_change = fn(value: String) {
    echo props.on_candidate_name_changed(team_name.new(value))
  }

  let on_add = props.on_team_added(model.candidate_name)

  div([class("team-names-editor__name-input")], [
    input([
      id("input-team-name"),
      placeholder("Add team"),
      tabindex(0),
      autofocus(True),
      value(team_name.to_string(model.candidate_name)),
      on_change(on_candidate_name_change),
    ]),
    button([tabindex(0), on_click(on_add)], [
      text("+"),
    ]),
  ])
}

fn teams_list(model: Model, props: Props(msg)) -> Element(msg) {
  div([class("team-names-editor__teams-table-wrapper")], [
    div(
      [class("team-names-editor__teams-table")],
      model.team_names
        |> list.index_map(fn(name, index) { team_row(name, index, props) }),
    ),
  ])
}

fn team_row(
  team_name: TeamName,
  index: Int,
  props: Props(msg),
) -> Element(msg) {
  let on_team_removed = props.on_team_removed(index)

  element.fragment([
    button([tabindex(0), on_click(on_team_removed)], [text("-")]),
    span([], [text(team_name.to_string(team_name))]),
  ])
}
