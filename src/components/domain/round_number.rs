use crate::domain::prelude::*;

use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn RoundNumberView(value: RoundNumber) -> Element {
    rsx! {
        Container {
            {t!("round-view")}
            { " " }
            { value.to_string() }
        }
    }
}

#[component]
fn Container(children: Element) -> Element {
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
