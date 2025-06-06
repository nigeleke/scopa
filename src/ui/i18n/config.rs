use dioxus_i18n::{prelude::*, *};
use unic_langid::{LanguageIdentifier, langid};

pub fn config(initial_language: LanguageIdentifier) -> I18nConfig {
    I18nConfig::new(initial_language)
        .with_locale((langid!("en-GB"), include_str!("../../../i18n/en-GB.ftl")))
        .with_locale((langid!("it-IT"), include_str!("../../../i18n/it-IT.ftl")))
        .with_locale((langid!("en"), include_str!("../../../i18n/en-GB.ftl")))
        .with_locale((langid!("it"), include_str!("../../../i18n/it-IT.ftl")))
        .with_fallback(langid!("en-GB"))
}
