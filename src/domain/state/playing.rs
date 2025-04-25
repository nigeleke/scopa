use super::traits::{HasHistory, HasTarget, HasTeams};
use crate::domain::{history::History, points::Points, teams::Teams};

#[derive(Debug)]
pub struct Playing {
    teams: Teams,
    history: History,
    target: Points,
}

impl Playing {
    pub const fn new(teams: Teams, history: History, target: Points) -> Self {
        Self {
            teams,
            history,
            target,
        }
    }

    pub fn into_parts(self) -> (Teams, History, Points) {
        (self.teams, self.history, self.target)
    }
}

impl HasTeams for Playing {
    fn teams(&self) -> &Teams {
        &self.teams
    }
}

impl HasHistory for Playing {
    fn history(&self) -> &History {
        &self.history
    }
}

impl HasTarget for Playing {
    fn target(&self) -> Points {
        self.target
    }
}
