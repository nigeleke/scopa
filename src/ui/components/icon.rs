use dioxus::prelude::*;
use dioxus_i18n::tid;

#[component]
fn Icon(
    src: String,
    alt: String,
    #[props(optional)] on_click: EventHandler<()>,
    #[props(extends = button, extends=GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/css/ui/icon.css") }
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
            src: asset!("/assets/images/icon-menu.png"),
            alt: tid!("menu-icon.alt-text"),
            popovertarget: if popovertarget.is_some() { popovertarget.expect("popover target exists") }
        }
    }
}

#[component]
pub fn UndoIcon(can_undo: bool, on_click: EventHandler<()>) -> Element {
    rsx! {
        Icon {
            id: "undo",
            on_click,
            src: asset!("/assets/images/icon-undo.png"),
            alt: tid!("undo-icon.alt-text"),
            disabled: !can_undo,
        }
    }
}

#[component]
pub fn RestartIcon(on_click: EventHandler<()>) -> Element {
    rsx! {
        Icon {
            on_click,
            src: asset!("/assets/images/icon-restart.png"),
            alt: tid!("restart-icon.alt-text"),
        }
    }
}

#[component]
pub fn HelpIcon(on_click: EventHandler<()>) -> Element {
    rsx! {
        Icon {
            on_click,
            src: asset!("/assets/images/icon-help.png"),
            alt: tid!("help-icon.alt-text"),
        }
    }
}

#[component]
pub fn HomeIcon(on_click: EventHandler<()>) -> Element {
    rsx! {
        Icon {
            on_click,
            src: asset!("/assets/images/icon-home.png"),
            alt: tid!("home-icon.alt-text"),
        }
    }
}
