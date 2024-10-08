use crate::domain::InternalGameState;
use crate::domain::prelude::*;
use crate::types::{Target, TeamId};

#[derive(Clone, Debug, PartialEq)]
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

    pub fn target(&self) -> Target {
        self.target    
    }

    pub fn winner(&self) -> TeamId {
        self.winner
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
