import lustre/attribute as a
import lustre/element.{type Element}
import lustre/element/html as h
import lustre/event

pub fn action(on_click: msg) -> Element(msg) {
  h.button([a.tabindex(0), event.on_click(on_click)], [
    h.text("Start again"),
  ])
}
