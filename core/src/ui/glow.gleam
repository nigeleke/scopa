import lustre/attribute as a
import lustre/element.{type Element}
import lustre/element/html as h

pub fn view(children: List(Element(msg))) -> Element(msg) {
  h.div([a.class("glow")], children)
}
