use serde::{Deserialize, Serialize};

use super::traits::{HasHistory, HasTarget, HasTeams};
use crate::domain::{history::History, target::Target, teams::Teams};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Playing {
    teams: Teams,
    history: History,
    target: Target,
}

impl Playing {
    pub const fn new(teams: Teams, history: History, target: Target) -> Self {
        Self {
            teams,
            history,
            target,
        }
    }

    pub fn into_parts(self) -> (Teams, History, Target) {
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
    fn target(&self) -> Target {
        self.target
    }
}
