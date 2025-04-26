use std::num::ParseIntError;

use serde::{Deserialize, Serialize};
use thiserror::*;

#[derive(Debug, Error)]
pub enum TargetError {
    #[error("no value")]
    NoValueProvided,

    #[error("invalid value")]
    InvalidValue(#[from] ParseIntError),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

impl std::str::FromStr for Target {
    type Err = TargetError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(TargetError::NoValueProvided)
        } else {
            let value = s.parse::<usize>()?;
            Ok(Self(value))
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
        self.0.fmt(f)
    }
}

#[coverage(off)]
#[cfg(test)]
mod test {
    use std::str::FromStr;

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn default_target_is_eleven() {
        assert_eq!(Target::default(), Target::from(11));
    }

    #[test]
    fn can_create_from_valid_string() {
        let p = Target::from_str("42").expect("valid target");
        assert_eq!(p, Target(42));
    }

    #[test]
    fn cannot_create_from_invalid_string() {
        let e = Target::from_str("forty two").expect_err("invalid target");
        assert!(matches!(e, TargetError::InvalidValue(_)));
        assert!(!matches!(e, TargetError::NoValueProvided));
    }

    #[test]
    fn cannot_create_from_empty_string() {
        let e = Target::from_str("").expect_err("invalid points");
        assert!(matches!(e, TargetError::NoValueProvided));
        assert!(!matches!(e, TargetError::InvalidValue(_)));
    }

    #[test]
    fn can_display_target() {
        assert_eq!(Target::from(42).to_string(), String::from("42"));
    }
}
