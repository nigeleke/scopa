import lustre/attribute
import lustre/element.{type Element}
import lustre/element/html
import lustre/event

pub fn action(on_click: msg) -> Element(msg) {
  html.button([attribute.tabindex(0), event.on_click(on_click)], [
    html.text("-"),
  ])
}
