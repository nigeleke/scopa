use dioxus_i18n::{prelude::*, *};
use unic_langid::{langid, LanguageIdentifier};

const DEFAULT_LANG_ID: LanguageIdentifier = langid!("en-GB");

pub fn config() -> I18nConfig {
    I18nConfig::new(DEFAULT_LANG_ID)
        .with_locale(Locale::new_static(
            langid!("en-GB"),
            include_str!("./en-GB.ftl"),
        ))
        .with_locale(Locale::new_static(
            langid!("it-IT"),
            include_str!("./it-IT.ftl"),
        ))
        .with_locale(Locale::new_static(
            langid!("en"),
            include_str!("./en-GB.ftl"),
        ))
        .with_locale(Locale::new_static(
            langid!("it"),
            include_str!("./it-IT.ftl"),
        ))
        .with_fallback(DEFAULT_LANG_ID)
}
