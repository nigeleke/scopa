use derive_more::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Display, From, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize,
)]
pub struct Target(pub usize);

impl Default for Target {
    fn default() -> Self {
        Self(11)
    }
}

impl TryFrom<String> for Target {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err("Required".into())
        } else {
            value
                .parse::<usize>()
                .map(|p| p.into())
                .map_err(|e| e.to_string())
        }
    }
}
