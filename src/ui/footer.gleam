import lustre/element.{type Element}
import lustre/element/html as h

pub fn view() -> Element(msg) {
  element.fragment([
    h.footer([], [h.text("Copyright (c) 2026; Nigel Eke. All rights reserved.")]),
  ])
}
