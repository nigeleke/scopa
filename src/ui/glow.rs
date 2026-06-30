use dioxus::prelude::*;

#[component]
pub fn Glow(children: Element) -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/css/glow.css") }
        div {
            class: "glow",
            { children }
        }
    }
}
