use std::convert::Infallible;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct TeamName(String);

impl From<&str> for TeamName {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

impl std::ops::Deref for TeamName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<String> for TeamName {
    type Error = Infallible;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(value))
    }
}

impl Into<String> for TeamName {
    fn into(self) -> String {
        self.0.to_string()
    }
}

impl std::fmt::Display for TeamName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
