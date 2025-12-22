use dioxus::prelude::*;
use dioxus_i18n::{prelude::*, unic_langid::langid};
use dioxus_sdk::storage::{LocalStorage, use_storage};

use crate::{
    domain::Target,
    ui::{
        components::{Errors, ScopaFooter, ScopaHeader},
        consts::{STORAGE_PAGE, STORAGE_STATE, STORAGE_TARGET},
        i18n::{config, use_user_preferred_language},
        pages::{Help, Home, Page},
        state::State,
    },
};

#[component]
pub fn App() -> Element {
    let _ = use_init_i18n(|| config(langid!("en-GB")));
    let language = use_user_preferred_language();
    provide_context(language);

    let default_target = use_storage::<LocalStorage, _>(STORAGE_TARGET.into(), Target::default);
    let state = use_storage::<LocalStorage, _>(STORAGE_STATE.into(), || {
        State::from(*default_target.read())
    });
    provide_context(state);

    let page = use_storage::<LocalStorage, _>(STORAGE_PAGE.into(), || Page::Home);
    provide_context(page);

    rsx! {
        document::Link { rel: "icon", href: asset!("/assets/favicon.ico") }
        document::Stylesheet { href: asset!("/assets/css/dx-components-theme.css") }
        document::Stylesheet { href: asset!("/assets/css/main.css") }
        document::Meta { name: "description", content: "Score your Scopa game using this website. This app is a free and easy to use program to help you score your Scopa card game." }
        document::Meta { name: "category", content: "game" }
        document::Meta { name: "keywords", content: "cards,game,scorer,scopa" }
        document::Meta { name: "author", content: "Nigel Eke" }

        ErrorBoundary {
            handle_error: |errors| rsx! { Errors { errors } },
            match page() {
                Page::Home => rsx! { Layout { Home { } } },
                Page::Help => rsx! { Layout { Help { } } },
            }
        }
    }
}

#[component]
fn Layout(children: Element) -> Element {
    rsx! {
        header { ScopaHeader {} }
        main { {children} }
        footer { ScopaFooter {} }
    }
}
