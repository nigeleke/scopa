use dioxus::prelude::{document::*, *};
use dioxus_i18n::t;

#[component]
fn Icon(
    src: String,
    alt: String,
    #[props(extends = button)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        Link { rel: "stylesheet", href: asset!("/assets/css/ui/icon.css") }
        button {
            ..attributes,
            img {
                class: "icon",
                src: src,
                alt: alt,
                height: "24px",
            }
        }
    }
}

#[component]
pub fn MenuIcon(popovertarget: Option<String>) -> Element {
    rsx! {
        Icon {
            src: asset!("assets/images/icon-menu-burger.png"),
            alt: t!("menu-icon-alt-text"),
            popovertarget: if popovertarget.is_some() { popovertarget.unwrap() }
        }
    }
}
