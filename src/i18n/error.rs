use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
    #[error("unknown language identifier {0}")]
    UnknownLanguageIdentifier(String),
}
