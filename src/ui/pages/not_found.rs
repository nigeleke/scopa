use dioxus::prelude::*;
use dioxus_i18n::tid;

#[component]
pub fn NotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { {tid!{"not-found.heading"}} }
        Link { to: "/", {tid!("not-found.home")} }
    }
}
