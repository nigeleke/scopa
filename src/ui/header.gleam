import lustre/element.{type Element}
import lustre/element/html as h

import ui/glow

pub fn view() -> Element(msg) {
  let header_content = [h.h1([], [h.text("Scopa Scorer")])]
  let header_content = glow.view(header_content)
  element.fragment([h.header([], [header_content])])
}
