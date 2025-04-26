use std::str::FromStr;

use dioxus::prelude::*;
use dioxus_i18n::tid;
use dioxus_sdk::storage::{LocalStorage, use_storage};

use super::{
    glow::Glow,
    icon::{MenuIcon, RestartIcon},
};
use crate::{
    domain::{Game, Target},
    ui::{
        consts::{MAIN_MENU_ID, STORAGE_TARGET},
        i18n::Language,
        state::State,
    },
};

#[component]
pub fn ScopaHeader() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/css/ui/scopa_header.css") }
        div {
            class: "scopa_header",
            Menu {}
            Glow { {tid!("scopa-app.title-text")} }
        }
    }
}

#[component]
fn Menu() -> Element {
    rsx! {
        nav {
            MenuIcon { popovertarget: MAIN_MENU_ID }
            div {
                class: "scopa_header_popover_menu",
                id: MAIN_MENU_ID,
                popover: "auto",
                Flag { src: asset!("/assets/images/flags/gb.svg"), lang: "en-GB" }
                Flag { src: asset!("/assets/images/flags/it.svg"), lang: "it-IT" }
                RestartMenuItem {}
            },
        }
    }
}

#[component]
fn Flag(src: String, lang: String) -> Element {
    let alt = tid!(&format!("lang.{}", lang));
    let lang = Language::from_str(lang.as_str()).ok();

    let mut i18n = use_context::<Signal<Option<Language>>>();

    let on_click = move |_| {
        i18n.set(lang.clone());
    };

    rsx! {
        button {
            onclick: on_click,
            img {
                class: "flag",
                src,
                alt,
            }
        }
    }
}

#[component]
fn RestartMenuItem() -> Element {
    let default_target = use_storage::<LocalStorage, _>(STORAGE_TARGET.into(), Target::default);
    let mut state = use_context::<Signal<State>>();

    let restart_game = move || state.set(State::from(Game::from(default_target())));

    match state() {
        State::Starting(_) => rsx! {},
        _ => rsx! { RestartIcon { on_click: restart_game } },
    }
}
