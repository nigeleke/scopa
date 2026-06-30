use std::ops::AddAssign;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Points(usize);

impl Points {
    pub fn zero() -> Self {
        Self(0)
    }

    pub fn value(&self) -> usize {
        self.0
    }
}

impl From<usize> for Points {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<bool> for Points {
    fn from(value: bool) -> Self {
        Self(if value { 1 } else { 0 })
    }
}

impl std::iter::Sum<Points> for Points {
    fn sum<I: Iterator<Item = Points>>(iter: I) -> Self {
        iter.fold(Points::zero(), |mut acc, points| {
            acc += points;
            acc
        })
    }
}

impl AddAssign<Points> for Points {
    fn add_assign(&mut self, rhs: Points) {
        self.0 += rhs.0
    }
}

impl std::fmt::Display for Points {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
