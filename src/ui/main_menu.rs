use dioxus::prelude::*;

use crate::application::Model;
use crate::i18n::Language;
use crate::ui::icon_button::{Icon, IconButton};

const MAIN_MENU_ID: &str = "main-menu";

#[component]
pub fn MainMenu() -> Element {
    let mut model = use_context::<Signal<Model>>();

    let fullscreen_allowed = web_sys::window()
        .and_then(|w| w.document())
        .map(|d| d.fullscreen_enabled())
        .unwrap_or_default();

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/main_menu.css") },
        div {
            class: "main-menu",
            div {
                class: "main-menu__root",
                IconButton {
                    popovertarget: MAIN_MENU_ID,
                    icon: Icon::Menu,
                    on_click: move |_| {}
                }
            }
            div {
                class: "main-menu__items",
                popover: "auto",
                id: MAIN_MENU_ID,
                IconButton {
                    icon: Icon::English,
                    on_click: move |_| model.write().set_language(Language::english()),
                }
                IconButton {
                    icon: Icon::Italian,
                    on_click: move |_| model.write().set_language(Language::italian()),
                }
                if fullscreen_allowed {
                    IconButton {
                        icon: Icon::Fullscreen,
                        on_click: move |_| {
                            if let Some(el) = web_sys::window()
                                .and_then(|w| w.document())
                                .and_then(|d| d.document_element())
                            {
                                spawn(async move { let _ = el.request_fullscreen(); });
                            }
                        },
                    }
                }
                if model.read().can_reset() {
                    IconButton {
                        icon: Icon::Reset,
                        on_click: move |_| model.write().reset(),
                    }
                }
            }
        }
    }
}
