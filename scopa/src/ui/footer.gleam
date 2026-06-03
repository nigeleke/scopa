import lustre/element.{type Element}
import lustre/element/html as h

import generated/version

pub fn view() -> Element(msg) {
  element.fragment([
    h.footer([], [
      h.text(
        "Copyright © 2025-2026; Nigel Eke. All rights reserved."
        <> " v"
        <> version.version,
      ),
    ]),
  ])
}
