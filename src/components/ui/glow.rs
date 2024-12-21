use dioxus::prelude::{document::*, *};

#[component]
pub fn Glow(
    children: Element,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        Link { rel: "stylesheet", href: asset!("/assets/css/ui/glow.css") }
        span {
            class: "glow",
            ..attributes,
            span { {children} }
        }
    }
}
