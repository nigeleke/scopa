import lustre/attribute
import lustre/element.{type Element}
import lustre/element/html
import lustre/event

import domain/team/name.{type TeamName}

pub fn view(name: TeamName) -> Element(msg) {
  html.span([attribute.class("team-name")], [
    html.text(name.to_string(name)),
  ])
}

pub fn editor(name: String, on_change: fn(String) -> msg) -> Element(msg) {
  html.input([
    attribute.class("team-name"),
    attribute.placeholder("Enter team name"),
    attribute.tabindex(0),
    event.on_change(on_change),
    attribute.value(name),
  ])
}
