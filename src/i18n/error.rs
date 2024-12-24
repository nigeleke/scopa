use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("unknown language identifier {0}")]
    UnknownLanguageIdentifier(String),
}
