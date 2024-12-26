use dioxus::prelude::{document::*, *};
use dioxus_i18n::t;

#[component]
pub fn ScopaFooter() -> Element {
    let version = env!("CARGO_PKG_VERSION");

    rsx! {
        Link { rel: "stylesheet", href: asset!("/assets/css/ui/scopa_footer.css") }
        div {
            class: "scopa_footer",
            p {
                {t!("scopa-copyright-text")},
                {t!("scopa-version-text", version: version)},
            }
        }
    }
}
