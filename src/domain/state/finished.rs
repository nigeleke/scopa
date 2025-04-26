use serde::{Deserialize, Serialize};

use super::traits::{HasHistory, HasTarget, HasTeams, HasWinner};
use crate::domain::{
    history::History,
    target::Target,
    teams::{TeamId, Teams},
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Finished {
    teams: Teams,
    history: History,
    winner: TeamId,
    target: Target,
}

impl Finished {
    pub const fn new(teams: Teams, history: History, winner: TeamId, target: Target) -> Self {
        Self {
            teams,
            history,
            winner,
            target,
        }
    }
}

impl HasTeams for Finished {
    fn teams(&self) -> &Teams {
        &self.teams
    }
}

impl HasHistory for Finished {
    fn history(&self) -> &History {
        &self.history
    }
}

impl HasWinner for Finished {
    fn winner(&self) -> TeamId {
        self.winner
    }
}

impl HasTarget for Finished {
    fn target(&self) -> Target {
        self.target
    }
}
