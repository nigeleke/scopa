import lustre/element.{type Element}
import lustre/element/html.{footer, text}

pub fn view() -> Element(msg) {
  element.fragment([
    footer([], [text("Copyright (c) 2026; Nigel Eke. All rights reserved.")]),
  ])
}
