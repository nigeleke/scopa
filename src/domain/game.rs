mod state;

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::domain::points::Points;
use crate::domain::round::{Round, RoundNumber};
use crate::domain::target::Target;
use crate::domain::team::{Team, TeamId, TeamName};

pub use state::GameState;

#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Game {
    state: GameState,
    target: Target,
    teams: Vec<Team>,
    rounds: Vec<Round>,
}

const MIN_TEAMS: usize = 2;
const MAX_TEAMS: usize = 6;

impl Game {
    pub fn target(&self) -> Target {
        self.target
    }

    pub fn set_target(&mut self, target: Target) {
        match &mut self.state {
            GameState::Setup => self.target = target,
            _ => panic!("invalid set_target in state {:?}", self.state),
        }
    }

    pub fn teams(&self) -> impl Iterator<Item = &Team> {
        self.teams.iter()
    }

    pub fn active_teams(&self) -> impl Iterator<Item = &Team> {
        self.teams.iter().filter(|t| t.is_playing())
    }

    pub fn leading_teams(&self) -> impl Iterator<Item = &Team> {
        self.teams
            .iter()
            .fold(BTreeMap::<Points, Vec<&Team>>::new(), |mut acc, team| {
                let score = self.points(*team.id());
                acc.entry(score).or_default().push(team);
                acc
            })
            .into_iter()
            .next_back()
            .into_iter()
            .flat_map(|(_, teams)| teams)
    }

    pub fn can_add_team(&self, name: &TeamName) -> bool {
        match &self.state {
            GameState::Setup => !name.is_empty() && self.teams.len() != MAX_TEAMS,
            _ => panic!("invalid can_add_team in state {:?}", self.state),
        }
    }

    pub fn add_team(&mut self, name: TeamName) {
        match &mut self.state {
            GameState::Setup => {
                let team = Team::new(name);
                self.teams.push(team);
            }
            _ => panic!("invalid add_team in state {:?}", self.state),
        }
    }

    pub fn remove_team(&mut self, id: TeamId) {
        match &mut self.state {
            GameState::Setup => self.teams.retain_mut(|team| team.id() != &id),
            _ => panic!("invalid remove_team in state {:?}", self.state),
        }
    }

    pub fn can_start(&self) -> bool {
        match &self.state {
            GameState::Setup => self.teams.len() >= MIN_TEAMS,
            _ => panic!("invalid can_start in state {:?}", self.state),
        }
    }

    pub fn start(&mut self) {
        match &self.state {
            GameState::Setup => self.state = GameState::Playing,
            GameState::Finished => {
                let mut game = Game {
                    teams: Vec::from_iter(self.teams().cloned()),
                    ..Default::default()
                };
                game.start();
                *self = game;
            }
            _ => panic!("invalid start in state {:?}", self.state),
        }
    }

    pub fn finish(&mut self) {
        match &self.state {
            GameState::Playing => self.state = GameState::Finished,
            _ => panic!("invalid finish in state {:?}", self.state),
        }
    }

    pub fn play(&mut self) {
        match &self.state {
            GameState::Finished => self.state = GameState::Playing,
            _ => panic!("invalid play in state {:?}", self.state),
        }
    }

    pub fn round_number(&self) -> RoundNumber {
        RoundNumber::from(self.rounds.len() + 1)
    }

    pub fn add_round(&mut self, round: Round) {
        self.rounds.push(round);
    }

    pub fn remove_round(&mut self) {
        let _ = self.rounds.pop();
    }

    pub fn last_round(&self) -> Option<&Round> {
        self.rounds.last()
    }

    pub fn points(&self, team_id: TeamId) -> Points {
        self.rounds.iter().map(|r| r.points(team_id)).sum()
    }

    pub fn state(&self) -> GameState {
        self.state
    }
}
