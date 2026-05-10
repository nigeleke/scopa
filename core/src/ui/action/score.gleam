import lustre/attribute as a
import lustre/element.{type Element}
import lustre/element/html as h
import lustre/event

import domain/tally.{type Tally}

pub fn action(tally: Tally, on_click: msg) -> Element(msg) {
  let can_score = tally.is_scoreable(tally)

  h.button(
    [
      a.tabindex(0),
      a.disabled(!can_score),
      event.on_click(on_click),
    ],
    [h.text("Score points")],
  )
}
