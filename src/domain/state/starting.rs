use super::traits::{HasTarget, HasTeams};
use crate::domain::{
    points::Points,
    teams::{Team, TeamId, Teams},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Starting {
    teams: Teams,
    target: Points,
}

impl Starting {
    pub fn into_parts(self) -> (Teams, Points) {
        (self.teams, self.target)
    }

    pub fn add_team(&mut self, team: Team) {
        self.teams.push(team);
    }

    pub fn remove_team(&mut self, id: TeamId) {
        self.teams.retain(|t| t.id() != id);
    }

    pub const fn set_target(&mut self, target: Points) {
        self.target = target;
    }

    const VALID_TEAM_COUNTS: [usize; 4] = [2, 3, 4, 6];

    pub fn can_start(&self) -> bool {
        Self::VALID_TEAM_COUNTS.contains(&self.teams.len())
    }
}

impl From<Points> for Starting {
    fn from(value: Points) -> Self {
        Self {
            teams: Teams::default(),
            target: value,
        }
    }
}

impl HasTeams for Starting {
    fn teams(&self) -> &Teams {
        &self.teams
    }
}

impl HasTarget for Starting {
    fn target(&self) -> Points {
        self.target
    }
}
