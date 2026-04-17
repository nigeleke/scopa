use dioxus::prelude::*;

#[component]
pub fn Glow(
    children: Element,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/css/ui/glow.css") }
        span {
            class: "glow",
            ..attributes,
            span { {children} }
        }
    }
}
