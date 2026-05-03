import lustre/attribute
import lustre/element.{type Element}
import lustre/element/html
import lustre/event

import domain/target.{type Target}

pub fn view(target: Target) -> Element(msg) {
  html.span([attribute.class("target")], [
    html.text(target.to_string(target)),
  ])
}

pub fn editor(
  raw_target: String,
  on_change: fn(String) -> msg,
) -> Element(msg) {
  html.div([attribute.class("target")], [
    html.div([attribute.class("target__input-wrapper")], [
      html.label([attribute.for("target-input")], [html.text("Target score: ")]),
      html.input([
        attribute.class("target__input"),
        attribute.id("target-input"),
        attribute.tabindex(0),
        attribute.inputmode("numeric"),
        attribute.type_("number"),
        attribute.min("1"),
        attribute.step("1"),
        attribute.value(raw_target),
        event.on_input(on_change),
      ]),
    ]),
    ..error_view(raw_target)
  ])
}

fn error_view(raw_target: String) -> List(Element(msg)) {
  case target.is_valid_target(raw_target) {
    True -> []
    False -> [
      html.span([attribute.class("target__error")], [
        html.text("Enter a valid target; usually 11, 16 or 21"),
      ]),
    ]
  }
}
