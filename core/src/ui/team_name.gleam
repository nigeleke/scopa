import lustre/attribute as a
import lustre/element.{type Element}
import lustre/element/html as h
import lustre/event

import domain/team/name.{type TeamName}

pub fn view(name: TeamName) -> Element(msg) {
  h.span([a.class("team-name")], [
    h.text(name.to_string(name)),
  ])
}

pub fn editor(name: String, on_change: fn(String) -> msg) -> Element(msg) {
  h.input([
    a.class("team-name"),
    a.placeholder("Enter team name"),
    a.tabindex(0),
    event.on_input(on_change),
    a.value(name),
  ])
}
