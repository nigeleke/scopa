#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RoundNumber(usize);

impl std::fmt::Display for RoundNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<usize> for RoundNumber {
    fn from(value: usize) -> Self {
        Self(value)
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
