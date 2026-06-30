use dioxus::prelude::*;

#[component]
pub fn IconButton(
    icon: String,
    on_click: EventHandler<()>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/css/icon_button.css") }
        button {
            class: "icon-button",
            onclick: move |_| on_click.call(()),
            ..attributes,
            {icon}
        }
    }
}
