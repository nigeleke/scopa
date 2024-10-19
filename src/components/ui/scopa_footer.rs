use dioxus::prelude::*;

#[component]
pub fn ScopaFooter() -> Element {
    let version = env!("CARGO_PKG_VERSION");
    
    rsx! {
        p {
            "Copyright © 2024; Nigel Eke. All rights reserved. ",
            "v" {version}
        }
    }
}