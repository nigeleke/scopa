use crate::types::RoundNumber;

use dioxus::prelude::*;

#[component]
pub fn RoundNumberView(
    value: RoundNumber
) -> Element {
    rsx! {
        Container {
            "Round number "
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