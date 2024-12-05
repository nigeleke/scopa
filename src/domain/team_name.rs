use derive_more::*;
use serde::{Deserialize, Serialize};

use std::convert::Infallible;

#[derive(Clone, Debug, Default, Deref, Display, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[deref(forward)]
pub struct TeamName(String);

impl From<&str> for TeamName {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

impl TryFrom<String> for TeamName {
    type Error = Infallible;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(value))
    }
}
