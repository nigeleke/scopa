use serde::Deserialize;
use serde::Serialize;

use crate::domain::prelude::*;
use crate::domain::InternalGameState;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FinishedState {
    target: Target,
    teams: Vec<Team>,
    rounds: Vec<Round>,
    winner: TeamId,
}

impl InternalGameState for FinishedState {}

impl FinishedState {
    pub fn new(teams: &[Team], rounds: &[Round], target: Target, winner: TeamId) -> Self {
        Self {
            teams: Vec::from(teams),
            rounds: Vec::from(rounds),
            target,
            winner,
        }
    }

    pub fn winner(&self) -> TeamName {
        self.teams()
            .iter()
            .find(|t| t.id() == self.winner)
            .map_or(TeamName::default(), |t| t.name())
    }
}

impl Teams for FinishedState {
    fn teams(&self) -> &[Team] {
        &self.teams
    }
}

impl Rounds for FinishedState {
    fn rounds(&self) -> &[Round] {
        &self.rounds
    }
}
