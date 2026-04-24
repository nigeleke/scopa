import lustre/attribute.{class, disabled}
import lustre/element.{type Element}
import lustre/element/html.{button, div, text}
import lustre/event.{on_click}

import domain/round.{type Round}
import domain/target.{type Target}
import domain/team_name.{type TeamName}

pub type Model {
  Model(rounds: List(Round), team_names: List(TeamName), target: Target)
}

pub type Props(msg) {
  Props(on_score: msg)
}

pub fn view(_model: Model, props: Props(msg)) -> Element(msg) {
  let on_score = props.on_score

  let enable_score = False

  div([class("in-progress")], [
    button([disabled(!enable_score), on_click(on_score)], [text("Score")]),
  ])
}

fn round(model: Model) -> Element(msg) {
  element.fragment([])
}
