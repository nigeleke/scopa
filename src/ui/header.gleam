import lustre/element.{type Element}
import lustre/element/html.{h1, header, text}

import ui/glow

pub fn view() -> Element(msg) {
  let header_content = [h1([], [text("Scopa Scorer")])]
  let header_content = glow.view(header_content)
  element.fragment([header([], [header_content])])
}
