use serde::{Deserialize, Serialize};

use super::{id::Id, name::Name};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Team {
    id: Id,
    name: Name,
    active: bool,
}

impl Team {
    pub const fn id(&self) -> Id {
        self.id
    }

    pub const fn is_playing(&self) -> bool {
        self.active
    }

    pub const fn is_not_playing(&self) -> bool {
        !self.is_playing()
    }

    pub const fn set_inactive(&mut self) {
        self.active = false
    }

    pub const fn name(&self) -> &Name {
        &self.name
    }
}

impl From<&str> for Team {
    fn from(value: &str) -> Self {
        Self {
            id: Id::default(),
            name: Name::from(value),
            active: true,
        }
    }
}

impl std::fmt::Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.name.fmt(f)
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::{assert_eq, assert_ne};

    use super::*;

    #[test]
    fn can_be_created() {
        let team = Team::from("Zaphod");
        assert_eq!(team.name(), &Name::from("Zaphod"));
    }

    #[test]
    fn will_be_created_with_unique_ids() {
        let team1 = Team::from("Zaphod");
        let team2 = Team::from("Zaphod");
        assert_ne!(team1, team2);
        assert_ne!(team1.id(), team2.id());
        assert_eq!(team1.name(), team2.name());
    }

    #[test]
    fn will_start_as_active() {
        let team = Team::from("Dr Who");
        assert!(team.is_playing());
        assert!(!team.is_not_playing());
    }

    #[test]
    fn can_become_inactive() {
        let mut team = Team::from("Dr Who");
        team.set_inactive();
        assert!(!team.is_playing());
        assert!(team.is_not_playing());
    }

    #[test]
    fn can_be_displayed() {
        let team = Team::from("Dr Who");
        assert_eq!(team.to_string(), String::from("Dr Who"));
    }
}
