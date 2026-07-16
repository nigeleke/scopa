use dioxus::prelude::*;

use std::str::FromStr;

use crate::i18n::Language;

pub fn use_preferred_language() -> Signal<Language> {
    use_signal(|| {
        web_sys::window()
            .and_then(|w| w.navigator().language())
            .and_then(|l| Language::from_str(&l).ok())
            .unwrap_or_else(Language::english)
    })
}
