use crate::domain::prelude::*;

use dioxus::prelude::*;

#[component]
pub fn RoundNumberView(
    value: RoundNumber
) -> Element {
    rsx! {
        Container {
            "Round "
             { value.to_string() }
        }
    }
}

#[component]
fn Container(
    children: Element,
) -> Element {
    rsx! {
        div {
            class: "round-number-outer",
            div {
                class: "round-number-inner",
                {children},
            },
        }
    }
}
