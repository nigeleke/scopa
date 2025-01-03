use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub struct Target(usize);

impl Target {
    pub fn value(&self) -> usize {
        self.0
    }
}

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

impl From<usize> for Target {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
