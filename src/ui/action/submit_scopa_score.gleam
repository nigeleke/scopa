import lustre/attribute as a
import lustre/element.{type Element}
import lustre/element/html as h
import lustre/event

import domain/score.{type Score}

pub fn action(score: Score, on_click: fn(Score) -> msg) -> Element(msg) {
  h.button([a.tabindex(0), event.on_click(on_click(score))], [
    h.text(score.to_string(score)),
  ])
}
