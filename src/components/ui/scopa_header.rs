use crate::components::prelude::*;

use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn ScopaHeader() -> Element {
    rsx! {
        span {
            class: "scopa_header",
            Glow { {t!("scopa-title-text")} }
        }
    }
}
