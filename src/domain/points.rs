use std::num::ParseIntError;

use thiserror::*;

#[derive(Debug, Error)]
pub enum PointsError {
    #[error("no value")]
    NoValueProvided,

    #[error("invalid value")]
    InvalidValue(#[from] ParseIntError),
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Points(usize);

impl std::str::FromStr for Points {
    type Err = PointsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(PointsError::NoValueProvided)
        } else {
            let value = s.parse::<usize>()?;
            Ok(Self(value))
        }
    }
}

impl From<usize> for Points {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl std::ops::Add for Points {
    type Output = Self;

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
    fn default_points_are_zero() {
        assert_eq!(Points::default(), Points::from(0));
    }

    #[test]
    fn can_create_from_valid_string() {
        let p = Points::from_str("42").expect("valid points");
        assert_eq!(p, Points(42));
    }

    #[test]
    fn cannot_create_from_invalid_string() {
        let e = Points::from_str("forty two").expect_err("invalid points");
        assert!(matches!(e, PointsError::InvalidValue(_)));
        assert!(!matches!(e, PointsError::NoValueProvided));
    }

    #[test]
    fn cannot_create_from_empty_string() {
        let e = Points::from_str("").expect_err("invalid points");
        assert!(matches!(e, PointsError::NoValueProvided));
        assert!(!matches!(e, PointsError::InvalidValue(_)));
    }

    #[test]
    fn can_add_points() {
        let one = Points::from(1);
        let two = Points::from(2);
        assert_eq!(one + two, Points::from(3));
    }

    #[test]
    fn can_add_assign_points() {
        let mut p = Points::from(1);
        p += Points::from(2);
        assert_eq!(p, Points::from(3));
    }

    #[test]
    fn can_order_points() {
        let mut ps = Vec::from([
            Points::from(42),
            Points::from(21),
            Points::from(0),
            Points::from(21),
        ]);
        ps.sort();
        assert_eq!(
            ps,
            Vec::from([
                Points::from(0),
                Points::from(21),
                Points::from(21),
                Points::from(42)
            ])
        );
    }

    #[test]
    fn can_display_points() {
        assert_eq!(Points::from(42).to_string(), String::from("42"));
    }
}
