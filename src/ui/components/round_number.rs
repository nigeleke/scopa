use dioxus::prelude::*;
use dioxus_i18n::tid;

use crate::domain::*;

#[component]
pub fn RoundNumberView(value: RoundNumber) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/css/domain/round_number.css") }
        Container {
            {tid!("round-view.text")}
            { " " }
            { value.to_string() }
        }
    }
}

#[component]
fn Container(children: Element) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/css/domain/round_number.css") }
        div {
            class: "round-number-outer",
            div {
                class: "round-number-inner",
                {children},
            },
        }
    }
}
