use crate::domain::{GameState, InternalGameState};
use crate::domain::prelude::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct StartingState {
    teams: Vec<Team>,
    target: Target,
}

impl InternalGameState for StartingState {}

impl StartingState {
    pub fn add_team(&mut self, team: Team) -> Result<()> {
        if let Some(existing_team) = self.find_team(team.id()) {
            Err(Error::TeamExists(existing_team.id()))
        } else {
            self.teams.push(team);
            Ok(())
        }
    }

    pub fn remove_team(&mut self, id: TeamId) -> Result<()> {
        if let Some(_existing_team) = self.find_team(id) {
            self.teams.retain(|t| t.id() != id);
            Ok(())
        } else { 
            Err(Error::TeamNotFound(id))
        }
    }

    pub fn target(&self) -> Target {
        self.target
    }

    pub fn set_target(&mut self, target: Target) {
        self.target = target;
    }

    const VALID_TEAM_COUNTS: [TeamCount; 4] = [TeamCount::new(2), TeamCount::new(3), TeamCount::new(4), TeamCount::new(6)];

    pub fn can_start(&self) -> bool {
        Self::VALID_TEAM_COUNTS.contains(&self.team_count())
    }

    pub fn start(&self) -> Result<GameState> {
        let team_count = self.team_count();
        if StartingState::VALID_TEAM_COUNTS.contains(&team_count) {
            Ok(GameState::Playing(Game::new_playing_state(&self.teams, self.target)))
        } else {
            Err(Error::InvalidNumberOfTeams(team_count))
        }

    }
}

impl Teams for StartingState {
    fn teams(&self) -> &[Team] {
        &self.teams
    }
}

impl Rounds for StartingState {
    fn rounds(&self) -> &[Round] {
        unreachable!()
    }
}

// impl HasTarget for StartingState {
//     fn target(&self) -> Target {
//         self.target
//     }
// }
