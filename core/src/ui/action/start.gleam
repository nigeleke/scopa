import lustre/attribute as a
import lustre/element.{type Element}
import lustre/element/html as h
import lustre/event

pub fn action(disabled: Bool, on_click: msg) -> Element(msg) {
  h.button([a.tabindex(0), a.disabled(disabled), event.on_click(on_click)], [
    h.text("Start"),
  ])
}
