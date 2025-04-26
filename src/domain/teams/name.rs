use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Name(String);

impl From<&str> for Name {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

impl std::fmt::Display for Name {
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
        let team = Name::from("Richard");
        assert_eq!(team, Name(String::from("Richard")))
    }

    #[test]
    fn can_be_displayed() {
        let team = Name::from("Feynman");
        assert_eq!(team.to_string(), String::from("Feynman"))
    }
}
