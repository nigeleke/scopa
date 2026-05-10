import lustre/attribute as a
import lustre/element.{type Element}
import lustre/element/html as h
import lustre/event

import domain/target.{type Target}

pub fn view(target: Target) -> Element(msg) {
  h.span([a.class("target")], [
    h.text(target.to_string(target)),
  ])
}

pub fn editor(
  raw_target: String,
  on_change: fn(String) -> msg,
) -> Element(msg) {
  h.div([a.class("target")], [
    h.div([a.class("target__input-wrapper")], [
      h.label([a.for("target-input")], [h.text("Target score: ")]),
      h.input([
        a.class("target__input"),
        a.id("target-input"),
        a.tabindex(0),
        a.inputmode("numeric"),
        a.type_("number"),
        a.min("1"),
        a.step("1"),
        a.value(raw_target),
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
      h.span([a.class("target__error")], [
        h.text("Enter a valid target; usually 11, 16 or 21"),
      ]),
    ]
  }
}
