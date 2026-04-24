import gleam/list
import lustre/attribute.{class, disabled}
import lustre/element.{type Element}
import lustre/element/html.{button, div, text}
import lustre/event.{on_click}

import domain/round.{type Round}
import domain/target.{type Target}
import domain/team.{type Team}

pub type Model {
  Model(teams: List(Team), rounds: List(Round), target: Target)
}

pub type Props(msg) {
  Props(on_score: fn(Round) -> msg)
}

pub fn view(_model: Model, props: Props(msg)) -> Element(msg) {
  let on_score = props.on_score(list.new())

  let enable_score = False

  div([class("in-progress")], [
    button([disabled(!enable_score), on_click(on_score)], [text("Score")]),
  ])
}
