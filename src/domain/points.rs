use crate::domain::prelude::Target;

use derive_more::*;

#[derive(Add, AddAssign, Clone, Copy, Debug, Default, Display, From, PartialEq, PartialOrd, Eq, Ord)]
pub struct Points(pub usize);

impl TryFrom<String> for Points {
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

impl PartialEq<Target> for Points {
    fn eq(&self, other: &Target) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd<Target> for Points {
    fn partial_cmp(&self, other: &Target) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
