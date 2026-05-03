import lustre/attribute
import lustre/element.{type Element}
import lustre/element/html
import lustre/event

import domain/tally.{type Tally}

pub fn action(tally: Tally, on_click: msg) -> Element(msg) {
  let can_score = tally.is_scoreable(tally)

  html.button(
    [
      attribute.tabindex(0),
      attribute.disabled(!can_score),
      event.on_click(on_click),
    ],
    [html.text("Score")],
  )
}
