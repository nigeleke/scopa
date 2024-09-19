#[derive(Clone, Copy, Debug)]
pub struct TeamCount(usize);

impl TeamCount {
    pub const fn new(value: usize) -> Self {
        Self(value)
    }
}

impl From<usize> for TeamCount {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl PartialEq for TeamCount {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TeamName(String);

impl From<&str> for TeamName {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TeamId(Uuid);

impl TeamId {
    pub fn new() -> Self {
        TeamId(Uuid::new_v4())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Points(usize);

impl Default for Points {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl From<usize> for Points {
    fn from(value: usize) -> Self {
        Self(value)
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
        self.0 += rhs.0;
    }
}
