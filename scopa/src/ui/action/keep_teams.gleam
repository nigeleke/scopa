import lustre/attribute as a
import lustre/element.{type Element}
import lustre/element/html as h
import lustre/event

pub fn action(
  keep_teams: Bool,
  on_update_keep_teams: fn(Bool) -> msg,
) -> Element(msg) {
  h.span([], [
    h.input([
      a.type_("checkbox"),
      a.checked(keep_teams),
      event.on_check(on_update_keep_teams),
    ]),
    h.text("Same teams"),
  ])
}
