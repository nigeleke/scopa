use dioxus::prelude::*;
use dioxus_i18n::{prelude::*, unic_langid::langid};
use dioxus_sdk::storage::{LocalStorage, use_storage};

use super::{
    consts::{STORAGE_STATE, STORAGE_TARGET},
    i18n::{config, use_user_preferred_language},
    routes::Route,
    state::State,
};
use crate::domain::Target;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/css/main.css");

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

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Meta { name: "description", content: "Score your Scopa game using this website. This app is a free and easy to use program to help you score your Scopa card game." }
        document::Meta { name: "category", content: "game" }
        document::Meta { name: "keywords", content: "cards,game,scorer,scopa" }
        document::Meta { name: "author", content: "Nigel Eke" }

        Router::<Route> {}
    }
}
