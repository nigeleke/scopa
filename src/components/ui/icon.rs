use dioxus::prelude::{document::*, *};
use dioxus_i18n::tid;

#[component]
fn Icon(
    src: String,
    alt: String,
    #[props(optional)] on_click: EventHandler<()>,
    #[props(extends = button)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        Link { rel: "stylesheet", href: asset!("/assets/css/ui/icon.css") }
        button {
            onclick: move |_| on_click.call(()),
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
            src: asset!("assets/images/icon-menu.png"),
            alt: tid!("menu-icon.alt-text"),
            popovertarget: if popovertarget.is_some() { popovertarget.unwrap() }
        }
    }
}

#[component]
pub fn RestartIcon(on_click: EventHandler<()>) -> Element {
    rsx! {
        Icon {
            on_click,
            src: asset!("assets/images/icon-restart.png"),
            alt: tid!("restart-icon.alt-text"),
        }
    }
}
