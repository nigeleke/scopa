use std::str::FromStr;

use dioxus_i18n::{prelude::*, *};
use unic_langid::{langid, LanguageIdentifier};

pub fn config(user_langid: &str) -> I18nConfig {
    let default_langid = LanguageIdentifier::from_str(user_langid)
        .ok()
        .unwrap_or(langid!("en-GB"));

    I18nConfig::new(default_langid)
        .with_locale(Locale::new_static(
            langid!("en-GB"),
            include_str!("./en-GB.ftl"),
        ))
        .with_locale(Locale::new_static(
            langid!("it-IT"),
            include_str!("./it-IT.ftl"),
        ))
        .with_fallback(langid!("en-GB"))
}
