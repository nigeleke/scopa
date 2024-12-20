use crate::domain::prelude::Target;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub struct Points(usize);

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

impl From<usize> for Points {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl PartialEq<Target> for Points {
    fn eq(&self, other: &Target) -> bool {
        self.0 == other.value()
    }
}

impl PartialOrd<Target> for Points {
    fn partial_cmp(&self, other: &Target) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.value())
    }
}

impl std::ops::Add for Points {
    type Output = Points;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::AddAssign for Points {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl std::fmt::Display for Points {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
