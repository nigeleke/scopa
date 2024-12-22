#![feature(variant_count)]

mod components;
mod domain;
mod i18n;
mod pages;
mod use_persistent;

use crate::pages::home::Home;

use dioxus::prelude::*;
use dioxus_i18n::prelude::use_init_i18n;
use dioxus_logger::tracing::Level;

fn main() {
    dioxus_logger::init(Level::DEBUG).expect("Cannot start logging");
    launch(app);
}

fn app() -> Element {
    let mut i18n = use_init_i18n(i18n::config);

    let document_language = use_resource(move || async move {
        let mut eval = document::eval("dioxus.send(navigator.language)");
        eval.recv::<String>().await.unwrap()
    });

    use_effect(move || {
        if let Some(l) = document_language() {
            i18n.set_language(i18n::langid(&l))
        }
    });

    rsx! { Home {} }
}
