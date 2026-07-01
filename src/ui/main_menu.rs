use dioxus::prelude::*;

use crate::application::Model;
use crate::i18n::Language;
use crate::ui::icon_button::IconButton;

const MAIN_MENU_ID: &str = "main-menu";

#[component]
pub fn MainMenu() -> Element {
    let mut model = use_context::<Signal<Model>>();

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/main_menu.css") },
        div {
            class: "main-menu",
            div {
                class: "main-menu__root",
                IconButton {
                    popovertarget: MAIN_MENU_ID,
                    icon: "\u{2630}",
                    on_click: move |_| {}
                }
            }
            div {
                class: "main-menu__items",
                popover: "auto",
                id: MAIN_MENU_ID,
                IconButton {
                    icon: "\u{1f1ec}\u{1f1e7}",
                    on_click: move |_| model.write().set_language(Language::english()),
                }
                IconButton {
                    icon: "\u{1f1ee}\u{1f1f9}",
                    on_click: move |_| model.write().set_language(Language::italian()),
                }
                IconButton {
                    icon: "\u{26f6}",
                    on_click: move |_| {
                        let _ =  document::eval("document.documentElement.requestFullscreen()");
                    },
                }
                if model.read().can_reset() {
                    IconButton {
                        icon: "\u{23fb}",
                        on_click: move |_| model.write().reset(),
                    }
                }
            }
        }
    }
}
