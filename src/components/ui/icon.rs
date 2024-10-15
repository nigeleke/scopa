use dioxus::prelude::*;

#[component]
pub fn Icon(
    src: String,
    #[props(default = true)]
    checked: bool,
    #[props(extends = img)]
    attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        div {
            class: "icon",
            class: if checked { "checked" },
            img {
                src,
                ..attributes,
            },
        }
    }
}