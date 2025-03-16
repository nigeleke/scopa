#![feature(variant_count)]

mod components;
mod domain;
mod i18n;
mod pages;

use i18n::Language;
use pages::home::Home;

use dioxus::prelude::*;
use dioxus_i18n::prelude::use_init_i18n;
use dioxus_i18n::unic_langid::langid;
use dioxus_logger::tracing::Level;
use dioxus_sdk::storage::{use_storage, LocalStorage};

fn main() {
    dioxus_logger::init(Level::DEBUG).expect("Cannot start logging");
    launch(app);
}

fn app() -> Element {
    let document_language = use_resource(move || async move {
        let mut eval = document::eval("dioxus.send(navigator.language)");
        Language::try_from(eval.recv::<String>().await.unwrap()).unwrap()
    });

    #[allow(clippy::redundant_closure)]
    let mut preferred_language =
        use_storage::<LocalStorage, _>("lang".into(), || document_language());
    let preferred_language_is_defined = use_signal(|| (*preferred_language.read()).is_some());

    use_effect(move || {
        if !*preferred_language_is_defined.read() {
            preferred_language.set(document_language());
        }
    });

    provide_context(preferred_language);

    let mut i18n = use_init_i18n(|| i18n::config(langid!("en-GB")));
    use_effect(move || {
        if let Some(preferred_language) = preferred_language.read().as_ref() {
            dioxus::logger::tracing::info!("Setting preferred language {:?}", preferred_language);
            i18n.set_language(preferred_language.identifier());
        }
    });

    rsx! { Home {} }
}
