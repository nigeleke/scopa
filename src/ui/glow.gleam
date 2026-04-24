import lustre/attribute.{class}
import lustre/element.{type Element}
import lustre/element/html.{div}

pub fn view(children: List(Element(msg))) -> Element(msg) {
  div([class("glow")], children)
}
