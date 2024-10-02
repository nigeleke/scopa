use crate::types::RoundNumber;

use dioxus::prelude::*;

#[component]
pub fn RoundNumberView(value: RoundNumber) -> Element {
    rsx! { { value.to_string() } }
}