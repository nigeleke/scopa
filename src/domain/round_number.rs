#[derive(Clone, Copy, PartialEq)]
pub struct RoundNumber(usize);

impl From<usize> for RoundNumber {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for RoundNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}