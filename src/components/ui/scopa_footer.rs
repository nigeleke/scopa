use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn ScopaFooter() -> Element {
    let version = env!("CARGO_PKG_VERSION");

    rsx! {
        p {
            {t!("scopa-copyright-text")},
            {t!("scopa-version-text", version: version)},
        }
    }
}
