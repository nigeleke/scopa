use crate::{components::prelude::*, i18n::Language};

use dioxus::prelude::{document::*, *};
use dioxus_i18n::t;

#[component]
pub fn ScopaHeader() -> Element {
    rsx! {
        Link { rel: "stylesheet", href: asset!("/assets/css/ui/scopa_header.css") }
        div {
            class: "scopa_header",
            Menu {}
            Glow { {t!("scopa-title-text")} }
        }
    }
}

#[component]
fn Menu() -> Element {
    const MAIN_MENU_ID: &str = "main-menu";
    rsx! {
        nav {
            MenuIcon { popovertarget: MAIN_MENU_ID }
            div {
                class: "scopa_header_popover_menu",
                id: MAIN_MENU_ID,
                popover: "auto",
                Flag { src: asset!("/assets/images/flags/gb.svg"), lang: "en-GB" }
                Flag { src: asset!("/assets/images/flags/it.svg"), lang: "it-IT" }
            },
        }
    }
}

#[component]
fn Flag(src: String, lang: String) -> Element {
    let alt = t!(&format!("lang-{}", lang));
    let lang = Language::try_from(lang).ok();

    let mut i18n = use_context::<Signal<Option<Language>>>();

    let on_click = move |_| {
        i18n.set(lang.clone());
    };

    rsx! {
        img {
            class: "flag",
            src,
            alt,
            onclick: on_click,
        }
    }
}
