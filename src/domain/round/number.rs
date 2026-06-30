#[derive(PartialEq, Eq)]
pub struct RoundNumber(usize);

impl RoundNumber {
    pub fn one() -> Self {
        Self(1)
    }
}

impl From<usize> for RoundNumber {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for RoundNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
