use dioxus::prelude::*;

#[component]
pub fn ScopaFooter() -> Element {
    rsx! {
        p { "Copyright © 2024; Nigel Eke. All rights reserved." }
    }
}