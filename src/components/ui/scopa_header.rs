use crate::components::prelude::*;

use dioxus::prelude::{document::*, *};
use dioxus_i18n::t;

#[component]
pub fn ScopaHeader() -> Element {
    rsx! {
        Link { rel: "stylesheet", href: asset!("/assets/css/ui/scopa_header.css") }
        span {
            class: "scopa_header",
            Glow { {t!("scopa-title-text")} }
        }
    }
}
