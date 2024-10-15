use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TeamId(Uuid);

impl TeamId {
    pub fn new() -> Self {
        TeamId(Uuid::new_v4())
    }
}

impl std::fmt::Display for TeamId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
