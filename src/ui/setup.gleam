import gleam/list
import lustre/attribute.{class, disabled}
import lustre/element.{type Element}
import lustre/element/html.{button, div, text}
import lustre/event.{on_click}

import domain/target.{type Target}
import domain/team_name.{type TeamName}
import ui/team_names_editor

pub type Model {
  Model(candidate_name: TeamName, team_names: List(TeamName), target: Target)
}

pub type Props(msg) {
  Props(
    on_candidate_name_changed: fn(TeamName) -> msg,
    on_team_added: fn(TeamName) -> msg,
    on_team_removed: fn(Int) -> msg,
    on_target_change: fn(Target) -> msg,
    on_start: msg,
  )
}

pub fn view(model: Model, props: Props(msg)) -> Element(msg) {
  let team_names_model =
    team_names_editor.Model(model.candidate_name, model.team_names)

  let team_names_props =
    team_names_editor.Props(
      props.on_candidate_name_changed,
      props.on_team_added,
      props.on_team_removed,
    )

  let on_start = props.on_start

  let enable_start =
    [2, 3, 4, 6] |> list.contains(model.team_names |> list.length)

  div([class("setup")], [
    team_names_editor.view(team_names_model, team_names_props),
    button([disabled(!enable_start), on_click(on_start)], [text("Start")]),
  ])
}
