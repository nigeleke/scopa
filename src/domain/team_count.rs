#[derive(Clone, Copy, Debug)]
pub struct TeamCount(usize);

impl TeamCount {
    pub const fn new(value: usize) -> Self {
        Self(value)
    }
}

impl PartialEq for TeamCount {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl std::fmt::Display for TeamCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
