import lustre/element.{type Element}
import lustre/element/html.{h1, header, text}

pub fn view() -> Element(msg) {
  element.fragment([header([], [h1([], [text("Scopa Scorer")])])])
}
