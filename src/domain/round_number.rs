#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RoundNumber(usize);

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

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn can_be_created() {
        assert_eq!(RoundNumber::from(42), RoundNumber(42));
    }

    #[test]
    fn can_be_displayed() {
        assert_eq!(RoundNumber(42).to_string(), String::from("42"));
    }
}
