use super::error::Error;

use dioxus_i18n::unic_langid::LanguageIdentifier;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Language(String);

impl Language {
    pub fn identifier(&self) -> LanguageIdentifier {
        LanguageIdentifier::from_bytes(self.0.as_bytes()).unwrap()
    }
}

impl TryFrom<String> for Language {
    type Error = super::I18nError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        LanguageIdentifier::from_bytes(value.as_bytes())
            .map(|_| Self(value))
            .map_err(|e| Error::UnknownLanguageIdentifier(e.to_string()))
    }
}
