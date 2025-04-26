use std::str::FromStr;

use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use dioxus_sdk::storage::*;

use super::language::Language;
use crate::ui::consts::STORAGE_LANGUAGE;

pub fn use_user_preferred_language() -> Signal<Option<Language>> {
    const FALLBACK_LANGUAGE: &str = "EN-GB";

    let document_language = use_resource(|| async {
        let mut eval = document::eval("dioxus.send(navigator.language)");
        let lang = eval
            .recv::<String>()
            .await
            .unwrap_or_else(|_| String::from(FALLBACK_LANGUAGE));
        Language::from_str(lang.as_str())
            .unwrap_or_else(|_| Language::from_str(FALLBACK_LANGUAGE).expect("valid fallback"))
    });

    #[allow(clippy::redundant_closure)]
    let mut preferred_language =
        use_storage::<LocalStorage, _>(STORAGE_LANGUAGE.into(), || document_language());
    let preferred_language_is_defined = use_signal(|| preferred_language.read().is_some());

    use_effect(move || {
        if !*preferred_language_is_defined.read() {
            preferred_language.set(document_language());
        }
    });

    let mut i18n = use_context::<I18n>();

    use_effect(move || {
        if let Some(lang) = preferred_language.read().as_ref() {
            i18n.set_language(lang.identifier());
        }
    });

    preferred_language
}
