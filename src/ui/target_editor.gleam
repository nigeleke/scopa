import lustre/attribute.{class}
import lustre/element.{type Element}
import lustre/element/html.{button, div, text}

import domain/target.{type Target}

pub type Model {
  Model(target: Target)
}

pub type Props(msg) {
  Props(on_target_changed: fn(Target) -> msg)
}

pub fn view(model: Model, _props: Props(msg)) -> Element(msg) {
  let message = "Play to " <> target.to_string(model.target) <> " points"
  div([class("target-editor")], [button([], [text(message)])])
}
