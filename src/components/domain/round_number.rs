use crate::domain::prelude::*;

use dioxus::prelude::{document::*, *};
use dioxus_i18n::t;

#[component]
pub fn RoundNumberView(value: RoundNumber) -> Element {
    rsx! {
        Link { rel: "stylesheet", href: asset!("/assets/css/domain/round_number.css") }
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
        Link { rel: "stylesheet", href: asset!("/assets/css/domain/round_number.css") }
        div {
            class: "round-number-outer",
            div {
                class: "round-number-inner",
                {children},
            },
        }
    }
}
