use dioxus::prelude::*;
use dioxus_i18n::tid;

#[component]
pub fn ScopaFooter() -> Element {
    let version = env!("CARGO_PKG_VERSION");

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/css/ui/scopa_footer.css") }
        div {
            class: "scopa_footer",
            p {
                {tid!("scopa-app.copyright-text")},
                {tid!("scopa-app.version-text", version: version)},
            }
        }
    }
}
