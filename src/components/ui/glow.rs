use dioxus::prelude::*;

#[component]
pub fn Glow(
    children: Element,
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        span {
            class: "glow",
            ..attributes,
            span { {children} }
        }
    }
}