use crate::components::prelude::*;

use dioxus::prelude::*;

#[component]
pub fn ScopaHeader() -> Element {
    rsx! {
        span {
            class: "scopa_header",
            Glow { "Scopa Scorer" }
        }
    }
}