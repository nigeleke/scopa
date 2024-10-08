use dioxus::prelude::*;

#[component]
pub fn ScopaHeader() -> Element {
    rsx! {
        span {
            class: "scopa_header",
            "Scopa Scorer"
        }
    }
}